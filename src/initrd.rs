// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
// SPDX-License-Identifier: BSD-3-Clause

use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::pin::Pin;
use core::ptr::{self};

use alloc::boxed::Box;
use uefi::boot::{self};
use uefi::proto::device_path::build::{media, DevicePathBuilder};
use uefi::{Handle, Result};
use uefi_raw::protocol::device_path::DevicePathProtocol;
use uefi_raw::protocol::media::LoadFile2Protocol;

use crate::FastbootBuffer;

unsafe extern "efiapi" fn load_file2_trampoline(
    this: *mut LoadFile2Protocol,
    _file_path: *const DevicePathProtocol,
    _boot_policy: uefi_raw::Boolean,
    buffer_size: *mut usize,
    buffer: *mut c_void,
) -> uefi_raw::Status {
    let this = this.cast::<LoadFile2ProtocolWrapper>().as_ref().unwrap();
    this.load_file(buffer_size, buffer.cast())
}

#[repr(C)]
struct LoadFile2ProtocolWrapper {
    protocol: LoadFile2Protocol,
    buffer: FastbootBuffer,
}

impl LoadFile2ProtocolWrapper {
    fn new(buffer: FastbootBuffer) -> Pin<Box<Self>> {
        let wrapper = Self {
            protocol: LoadFile2Protocol {
                load_file: load_file2_trampoline,
            },
            buffer,
        };
        Box::pin(wrapper)
    }

    fn load_file(&self, buf_len: *mut usize, buf: *mut c_void) -> uefi_raw::Status {
        unsafe { *buf_len = self.buffer.len() };

        if buf.is_null() || unsafe { *buf_len } < self.buffer.len() {
            uefi_raw::Status::BUFFER_TOO_SMALL
        } else {
            unsafe {
                ptr::copy_nonoverlapping(self.buffer.ptr.as_ptr(), buf.cast(), self.buffer.len());
            }
            uefi_raw::Status::SUCCESS
        }
    }
}

pub struct LinuxInitrd {
    load_file: Pin<Box<LoadFile2ProtocolWrapper>>,
    device_path: Box<[MaybeUninit<u8>; 256]>,
    handle: Handle,
}

impl LinuxInitrd {
    pub(crate) fn new(ramdisk: FastbootBuffer) -> Self {
        let mut load_file = LoadFile2ProtocolWrapper::new(ramdisk);
        let initrd_loadfile2_ptr = load_file.as_mut().get_mut();

        let handle = unsafe {
            boot::install_protocol_interface(
                None,
                &LoadFile2Protocol::GUID,
                (&raw const *initrd_loadfile2_ptr).cast(),
            )
        }
        .expect("Failed to install LoadFile2Protocol");

        let device_path = Box::new(Self::build_initrd_device_path().unwrap());
        unsafe {
            boot::install_protocol_interface(
                Some(handle),
                &DevicePathProtocol::GUID,
                device_path.as_ptr().cast(),
            )
        }
        .expect("Failed to instasll DevicePathProtocol");

        Self {
            load_file,
            device_path,
            handle,
        }
    }

    fn build_initrd_device_path() -> Result<[core::mem::MaybeUninit<u8>; 256]> {
        let mut dp_buf = [MaybeUninit::uninit(); 256];
        DevicePathBuilder::with_buf(&mut dp_buf)
            .push(&media::Vendor {
                vendor_guid: crate::LINUX_EFI_INITRD_MEDIA_GUID,
                vendor_defined_data: &[],
            })
            .unwrap()
            .finalize()
            .unwrap();

        Ok(dp_buf)
    }
}

impl Drop for LinuxInitrd {
    fn drop(&mut self) {
        let initrd_loadfile2_ptr = self.load_file.as_mut().get_mut();

        unsafe {
            boot::uninstall_protocol_interface(
                self.handle,
                &DevicePathProtocol::GUID,
                self.device_path.as_ptr().cast(),
            )
            .expect("Failed to uninstall initrd DevicePathProtocol");
            boot::uninstall_protocol_interface(
                self.handle,
                &LoadFile2Protocol::GUID,
                (&raw const *initrd_loadfile2_ptr).cast(),
            )
            .expect("Failed to uninstall initrd LoadFile2Protocol");
        }
    }
}
