// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
// SPDX-License-Identifier: BSD-3-Clause

#![no_main]
#![no_std]

extern crate alloc;

use alloc::{format, slice};
use core::ffi::c_void;
use core::ptr::{self, NonNull};
use log::info;
use uefi::boot::{EventType, MemoryType, ScopedProtocol, Tpl};
use uefi::data_types::Event;
use uefi::runtime::ResetType;
use uefi::{guid, prelude::*, CStr16, CString16, Error, Guid, Result};

use memcardinfo::MemCardInfo;

mod abootimg;
use abootimg::{handle_bootimg, is_bootimg};

mod initrd;
mod memcardinfo;

mod peimage;
use peimage::{handle_peimage, is_peimage};

mod proto;

mod usb_device;

use usb_device::EfiUsbDevice;

const QCOM_INIT_USB_CONTROLLER_GUID: Guid = guid!("1c0cffce-fc8d-4e44-8c78-9c9e5b530d36");
const EFI_RT_PROPERTIES_TABLE: Guid = guid!("eb66918a-7eef-402a-842e-931d21c38ae9");
const EFI_FDT_TABLE: Guid = guid!("b1b621d5-f19c-41a5-830b-d9152c69aae0");
const LINUX_EFI_INITRD_MEDIA_GUID: Guid = guid!("5568e427-68fc-4f3d-ac74-ca555231cc68");

extern "efiapi" fn dummy_callback(_: Event, _: Option<NonNull<c_void>>) {}

fn signal_usb_controller_init() -> Result {
    let event = unsafe {
        boot::create_event_ex(
            EventType::NOTIFY_SIGNAL,
            Tpl::CALLBACK,
            Some(dummy_callback),
            None,
            Some(NonNull::from(&QCOM_INIT_USB_CONTROLLER_GUID)),
        )?
    };
    boot::signal_event(&event)?;
    boot::close_event(event)?;

    Ok(())
}

fn fastboot_open(serial_number: &CStr16) -> Result<ScopedProtocol<EfiUsbDevice>> {
    let handle = boot::get_handle_for_protocol::<EfiUsbDevice>()?;
    let usb_device = boot::open_protocol_exclusive::<EfiUsbDevice>(handle)?;

    usb_device.start_ex(serial_number)?;

    Ok(usb_device)
}

fn fastboot_respond(usb_device: &ScopedProtocol<EfiUsbDevice>, response: &str) -> Result {
    let buf = usb_device
        .allocate_transfer_buffer(64)
        .expect("failed to allocate command buffer");

    let mut payload = response.as_bytes().to_vec();
    payload.push(0);

    unsafe {
        ptr::copy_nonoverlapping(payload.as_ptr(), buf, payload.len());
    }

    usb_device
        .send(usb_device::ENDPOINT_IN, payload.len(), buf)
        .expect("failed to send response");

    Ok(())
}

fn handle_download(usb_device: &ScopedProtocol<EfiUsbDevice>, size: usize) -> Result<&[u8]> {
    let mut download_remains = size;

    let target = boot::allocate_pool(MemoryType::BOOT_SERVICES_DATA, size).unwrap();
    let target_slice = unsafe { slice::from_raw_parts_mut(target.as_ptr(), size) };
    let mut offset = 0;

    let receive_buffer_size = size.min(16 * 1024 * 1024);

    let receive_buffer = usb_device
        .allocate_transfer_buffer(16 * 1024 * 1024)
        .expect("failed to allocate command buffer");

    fastboot_respond(usb_device, &format!("DATA{size:08x}"))?;

    usb_device
        .send(
            usb_device::ENDPOINT_OUT,
            receive_buffer_size,
            receive_buffer,
        )
        .expect("failed to queue command buffer");

    loop {
        let event = usb_device.handle_event().expect("handle_event failed");

        match event {
            usb_device::EfiUsbDeviceEvent::NoEvent => continue,
            usb_device::EfiUsbDeviceEvent::Connected => todo!(),
            usb_device::EfiUsbDeviceEvent::Disconnected => break,
            usb_device::EfiUsbDeviceEvent::OutData(data) => {
                let dest_slice = &mut target_slice[offset..offset + data.len()];
                dest_slice.copy_from_slice(data);

                download_remains -= data.len();
                offset += data.len();

                if offset == target_slice.len() {
                    break;
                }

                let next_chunk = download_remains.min(receive_buffer_size);
                usb_device
                    .send(usb_device::ENDPOINT_OUT, next_chunk, receive_buffer)
                    .expect("failed to queue command buffer");
            }
        }
    }

    usb_device.free_transfer_buffer(receive_buffer)?;

    if offset == target_slice.len() {
        fastboot_respond(usb_device, "OKAY")?;
    }

    Ok(target_slice)
}

struct FastbootBuffer {
    ptr: NonNull<u8>,
    len: usize,
    offset: usize,
}

impl FastbootBuffer {
    fn alloc(memory_type: MemoryType, size: usize) -> Result<Self> {
        let ptr = boot::allocate_pool(memory_type, size).unwrap();

        Ok(FastbootBuffer {
            ptr,
            len: size,
            offset: 0,
        })
    }

    fn len(&self) -> usize {
        self.len
    }

    fn write(&mut self, data: &[u8]) -> Result {
        if self.offset + data.len() > self.len {
            return Err(Error::new(Status::OUT_OF_RESOURCES, ()));
        }

        unsafe {
            ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.ptr.add(self.offset).as_ptr(),
                data.len(),
            )
        };

        self.offset += data.len();

        Ok(())
    }

    fn install_configuration_table(&self, guid: &'static Guid) -> Result {
        unsafe { boot::install_configuration_table(guid, self.ptr.as_ptr().cast())? };
        Ok(())
    }

    fn load_image(&self) -> Result<Handle> {
        let source = boot::LoadImageSource::FromBuffer {
            buffer: unsafe { slice::from_raw_parts(self.ptr.as_ref(), self.len) },
            file_path: None,
        };
        let handle = boot::load_image(boot::image_handle(), source)?;

        Ok(handle)
    }
}

#[repr(C)]
struct EfiRtPropertiesTable {
    version: u16,
    length: u16,
    runtime_services_supported: u32,
}

fn create_empty_rt_properties_table() -> Result<FastbootBuffer> {
    let table = EfiRtPropertiesTable {
        version: 1,
        length: 8,
        runtime_services_supported: 0,
    };

    let slice = unsafe {
        slice::from_raw_parts(
            (&raw const table).cast(),
            core::mem::size_of::<EfiRtPropertiesTable>(),
        )
    };

    let mut buf = FastbootBuffer::alloc(
        MemoryType::RUNTIME_SERVICES_DATA,
        core::mem::size_of::<EfiRtPropertiesTable>(),
    )?;
    buf.write(slice)?;

    Ok(buf)
}

fn handle_boot(usb_device: &ScopedProtocol<EfiUsbDevice>, payload: &[u8]) -> Result {
    let (handle, _initrd) = if is_peimage(payload) {
        (handle_peimage(payload)?, None)
    } else if is_bootimg(payload) {
        handle_bootimg(payload)?
    } else {
        fastboot_respond(usb_device, "FAIL")?;
        return Err(uefi::Error::new(Status::INVALID_PARAMETER, ()));
    };

    create_empty_rt_properties_table()?.install_configuration_table(&EFI_RT_PROPERTIES_TABLE)?;

    fastboot_respond(usb_device, "OKAY")?;
    boot::start_image(handle)?;

    Ok(())
}

fn generate_serial_number() -> Result<CString16> {
    let handle = boot::get_handle_for_protocol::<MemCardInfo>()?;
    let memcardinfo = boot::open_protocol_exclusive::<MemCardInfo>(handle)?;
    let cardinfo = memcardinfo.get_card_info()?;

    let serial = if cardinfo.card_type[0..3] == [b'U', b'F', b'S'] {
        let serial_number_len = cardinfo.serial_number_len as usize;
        boot::calculate_crc32(&cardinfo.serial_number[..serial_number_len])?
    } else {
        u32::from_le_bytes(cardinfo.serial_number[0..4].try_into().unwrap())
    };

    let mut buf = [0; 9];
    let serial = format!("{serial:08x}");
    let serial = CStr16::from_str_with_buf(&serial, &mut buf).unwrap();

    Ok(serial.into())
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let version = env!("BUILD_VERSION");
    info!("fastboot.efi {}", version);

    let serial_number = generate_serial_number().unwrap();

    signal_usb_controller_init().expect("failed to signal usb controller initialization");

    let usb_device = fastboot_open(&serial_number).expect("unable to open USB device");

    let command_buffer = usb_device
        .allocate_transfer_buffer(1024 * 1024)
        .expect("failed to allocate command buffer");

    let mut loaded_data: Option<&[u8]> = None;

    'message_loop: loop {
        let event = usb_device.handle_event().expect("handle_event failed");

        match event {
            usb_device::EfiUsbDeviceEvent::NoEvent => {}
            usb_device::EfiUsbDeviceEvent::Connected => {
                usb_device
                    .send(usb_device::ENDPOINT_OUT, 1024 * 1024, command_buffer)
                    .expect("failed to queue command buffer");
            }
            usb_device::EfiUsbDeviceEvent::OutData(data) => {
                let request = core::str::from_utf8(data).unwrap();

                if request.starts_with("download:") {
                    let parts = request.split(':').nth(1).unwrap();
                    let size = usize::from_str_radix(parts, 16).unwrap();

                    loaded_data = Some(handle_download(&usb_device, size).unwrap());
                } else if request == "boot" {
                    if let Some(payload) = loaded_data {
                        handle_boot(&usb_device, payload).expect("Failed to handle boot command");
                    } else {
                        fastboot_respond(&usb_device, "FAILdownload something first")
                            .expect("Failed to send response");
                    };
                } else if request == "reboot" {
                    let _ = fastboot_respond(&usb_device, "OKAY");

                    let reset_data = cstr16!("RESET_PARAM");
                    runtime::reset(
                        ResetType::COLD,
                        Status::SUCCESS,
                        Some(reset_data.as_bytes()),
                    );
                } else if request == "continue" {
                    let _ = fastboot_respond(&usb_device, "OKAY");

                    break 'message_loop;
                } else {
                    fastboot_respond(&usb_device, "FAILunknown command")
                        .expect("Failed to send response");
                }

                usb_device
                    .send(usb_device::ENDPOINT_OUT, 1024 * 1024, command_buffer)
                    .expect("failed to queue command buffer");
            }
            _ => info!("{:#?}", event),
        };
    }

    usb_device.stop().expect("Failed to stop USB");
    usb_device
        .free_transfer_buffer(command_buffer)
        .expect("Failed to free transfer buffer");

    Status::SUCCESS
}
