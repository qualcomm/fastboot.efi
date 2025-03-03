// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
// SPDX-License-Identifier: BSD-3-Clause

use alloc::boxed::Box;
use core::ptr::{self, slice_from_raw_parts};
use uefi::{proto::unsafe_protocol, CStr16, Result, StatusExt};

use crate::proto::usb_device::*;

pub const ENDPOINT_IN: u8 = 0x81;
pub const ENDPOINT_OUT: u8 = 0x1;

#[derive(Debug)]
#[repr(transparent)]
#[unsafe_protocol(EfiUsbDeviceProtocol::GUID)]
pub struct EfiUsbDevice(EfiUsbDeviceProtocol);

const DEVICE_DESCRIPTOR: UsbDeviceDescriptor = UsbDeviceDescriptor {
    bLength: USB_DEVICE_DESCRIPTOR_LEN,
    bDescriptorType: USB_DEVICE_DESCRIPTOR_TYPE,
    bcdUSB: 0x210,
    bDeviceClass: 0,
    bDeviceSubClass: 0,
    bDeviceProtocol: 0,
    bMaxPacketSize0: 64,
    idVendor: 0x18d1,
    idProduct: 0xd00d,
    bcdDevice: 0x100,
    iManufacturer: 1,
    iProduct: 2,
    iSerialNumber: 3,
    bNumConfigurations: 1,
};

const SUPERSPEED_DEVICE_DESCRIPTOR: UsbDeviceDescriptor = UsbDeviceDescriptor {
    bLength: USB_DEVICE_DESCRIPTOR_LEN,
    bDescriptorType: USB_DEVICE_DESCRIPTOR_TYPE,
    bcdUSB: 0x300,
    bDeviceClass: 0,
    bDeviceSubClass: 0,
    bDeviceProtocol: 0,
    bMaxPacketSize0: 9,
    idVendor: 0x18d1,
    idProduct: 0xd00d,
    bcdDevice: 0x100,
    iManufacturer: 1,
    iProduct: 2,
    iSerialNumber: 3,
    bNumConfigurations: 1,
};

const DEVICE_QUALIFIER: UsbDeviceQualifierDescriptor = UsbDeviceQualifierDescriptor {
    bLength: USB_DEVICE_QUALIFIER_DESCRIPTOR_LEN,
    bDescriptorType: USB_DEVICE_QUALIFIER_DESCRIPTOR_TYPE,
    bcdUSB: 0x200,
    bDeviceClass: 0xff,
    bDeviceSubClass: 0xff,
    bDeviceProtocol: 0xff,
    bMaxPacketSize0: 64,
    bNumConfigurations: 1,
    bReserved: 0,
};

const CONFIG_DESCRIPTOR_TREE: ConfigDescriptorTree = ConfigDescriptorTree {
    config_descriptor: UsbConfigDescriptor {
        bLength: USB_CONFIG_DESCRIPTOR_LEN,
        bDescriptorType: USB_CONFIG_DESCRIPTOR_TYPE,
        wTotalLength: (USB_CONFIG_DESCRIPTOR_LEN
            + USB_INTERFACE_DESCRIPTOR_LEN
            + USB_ENDPOINT_DESCRIPTOR_LEN
            + USB_ENDPOINT_DESCRIPTOR_LEN) as u16,
        bNumInterfaces: 1,
        bConfigurationValue: 1,
        iConfiguration: 0,
        bmAttributes: 0x80,
        bMaxPower: 0x50,
    },
    interface_descriptor: UsbInterfaceDescriptor {
        bLength: USB_INTERFACE_DESCRIPTOR_LEN,
        bDescriptorType: USB_INTERFACE_DESCRIPTOR_TYPE,
        bInterfaceNumber: 0,
        bAlternateSetting: 0,
        bNumEndpoints: 2,
        bInterfaceClass: 0xff,
        bInterfaceSubClass: 0x42,
        bInterfaceProtocol: 0x03,
        iInterface: 4,
    },
    endpoint0_descriptor: UsbEndpointDescriptor {
        bLength: USB_ENDPOINT_DESCRIPTOR_LEN,
        bDescriptorType: USB_ENDPOINT_DESCRIPTOR_TYPE,
        bEndpointAddress: ENDPOINT_IN,
        bmAttributes: USB_ENDPOINT_TYPE_BULK,
        wMaxPacketSize: 512,
        bInterval: 0,
    },
    endpoint1_descriptor: UsbEndpointDescriptor {
        bLength: USB_ENDPOINT_DESCRIPTOR_LEN,
        bDescriptorType: USB_ENDPOINT_DESCRIPTOR_TYPE,
        bEndpointAddress: ENDPOINT_OUT,
        bmAttributes: USB_ENDPOINT_TYPE_BULK,
        wMaxPacketSize: 512,
        bInterval: 1,
    },
};

const CONFIG_DESCRIPTOR_TREES: [*const ConfigDescriptorTree; 1] = [&CONFIG_DESCRIPTOR_TREE];

const SUPERSPEED_CONFIG_DESCRIPTOR_TREE: SuperSpeedConfigDescriptorTree =
    SuperSpeedConfigDescriptorTree {
        config_descriptor: UsbConfigDescriptor {
            bLength: USB_CONFIG_DESCRIPTOR_LEN,
            bDescriptorType: USB_CONFIG_DESCRIPTOR_TYPE,
            wTotalLength: (USB_CONFIG_DESCRIPTOR_LEN
                + USB_INTERFACE_DESCRIPTOR_LEN
                + USB_ENDPOINT_DESCRIPTOR_LEN
                + USB_SUPER_SPEED_COMPANION_DESCRIPTOR_LEN
                + USB_ENDPOINT_DESCRIPTOR_LEN
                + USB_SUPER_SPEED_COMPANION_DESCRIPTOR_LEN) as u16,
            bNumInterfaces: 1,
            bConfigurationValue: 1,
            iConfiguration: 0,
            bmAttributes: 0x80,
            bMaxPower: 0x10,
        },
        interface_descriptor: UsbInterfaceDescriptor {
            bLength: USB_INTERFACE_DESCRIPTOR_LEN,
            bDescriptorType: USB_INTERFACE_DESCRIPTOR_TYPE,
            bInterfaceNumber: 0,
            bAlternateSetting: 0,
            bNumEndpoints: 2,
            bInterfaceClass: 0xff,
            bInterfaceSubClass: 0x42,
            bInterfaceProtocol: 0x03,
            iInterface: 4,
        },
        endpoint0_descriptor: UsbEndpointDescriptor {
            bLength: USB_ENDPOINT_DESCRIPTOR_LEN,
            bDescriptorType: USB_ENDPOINT_DESCRIPTOR_TYPE,
            bEndpointAddress: ENDPOINT_IN,
            bmAttributes: USB_ENDPOINT_TYPE_BULK,
            wMaxPacketSize: 1024,
            bInterval: 0,
        },
        superspeed_ednpoint0_compaion_descriptor: UsbSuperSpeedCompanionDescriptor {
            bLength: USB_SUPER_SPEED_COMPANION_DESCRIPTOR_LEN,
            bDescriptorType: USB_SUPER_SPEED_COMPANION_DESCRIPTOR_TYPE,
            bMaxBurst: 4,
            bmAttributes: 0,
            wBytesPerInterval: 0,
        },
        endpoint1_descriptor: UsbEndpointDescriptor {
            bLength: USB_ENDPOINT_DESCRIPTOR_LEN,
            bDescriptorType: USB_ENDPOINT_DESCRIPTOR_TYPE,
            bEndpointAddress: ENDPOINT_OUT,
            bmAttributes: USB_ENDPOINT_TYPE_BULK,
            wMaxPacketSize: 1024,
            bInterval: 0,
        },
        superspeed_ednpoint1_compaion_descriptor: UsbSuperSpeedCompanionDescriptor {
            bLength: USB_SUPER_SPEED_COMPANION_DESCRIPTOR_LEN,
            bDescriptorType: USB_SUPER_SPEED_COMPANION_DESCRIPTOR_TYPE,
            bMaxBurst: 4,
            bmAttributes: 0,
            wBytesPerInterval: 0,
        },
    };

const SUPERSPEED_CONFIG_DESCRIPTOR_TREES: [*const SuperSpeedConfigDescriptorTree; 1] =
    [&SUPERSPEED_CONFIG_DESCRIPTOR_TREE];

const BINARY_OBJECT_STORE: BinaryObjectStore = BinaryObjectStore {
    descriptor: UsbBinaryObjectStoreDescriptor {
        bLength: USB_BINARY_OBJECT_STORE_DESCRIPTOR_LEN,
        bDescriptorType: USB_BINARY_OBJECT_STORE_DESCRIPTOR_TYPE,
        wTotalLength: (USB_BINARY_OBJECT_STORE_DESCRIPTOR_LEN
            + USB_DEVICE_CAPABLITY_USB20_EXTENSION_DESCRIPTOR_LEN
            + USB_DEVICE_CAPABLITY_SUPER_SPEED_DESCRIPTOR_LEN
            + USB_DEVICE_CAPABLITY_SUPER_SPEED_PLUS_DESCRIPTOR_LEN) as u16,
        bNumDeviceCaps: 3,
    },
    usb_20_descriptor: UsbDeviceCapablityUsb20ExtensionDescriptor {
        bLength: USB_DEVICE_CAPABLITY_USB20_EXTENSION_DESCRIPTOR_LEN,
        bDescriptorType: USB_DEVICE_CAPABLITY_DESCRIPTOR_TYPE,
        bDevCapabilityType: USB_DEVICE_CAPABLITY_USB20_EXTENSION_DESCRIPTOR_CAPABILITY_TYPE,
        bmAttributes: 0x6,
    },
    superspeed_capablity_descriptor: UsbDeviceCapablitySuperSpeedDescriptor {
        bLength: USB_DEVICE_CAPABLITY_SUPER_SPEED_DESCRIPTOR_LEN,
        bDescriptorType: USB_DEVICE_CAPABLITY_DESCRIPTOR_TYPE,
        bDevCapabilityType: USB_DEVICE_CAPABLITY_SUPER_SPEED_DESCRIPTOR_CAPABILTY_TYPE,
        bmAttributes: 0,
        wSpeedsSupported: 0xe,
        bFunctionalitySupport: ENDPOINT_OUT,
        bU1DevExitLat: 0x07,
        wU2DevExitLat: 0x65,
    },
    superspeed_plus_capability_descriptor: UsbDeviceCapablitySuperSpeedPlusDescriptor {
        bLength: USB_DEVICE_CAPABLITY_SUPER_SPEED_PLUS_DESCRIPTOR_LEN,
        bDescriptorType: USB_DEVICE_CAPABLITY_DESCRIPTOR_TYPE,
        bDevCapabilityType: USB_DEVICE_CAPABLITY_SUPER_SPEED_PLUS_DESCRIPTOR_CAPABILITY_TYPE,
        bReserved: 0,
        bmAttributes: 0x1,
        wFunctionalitySupport: 0x1100,
        wReserved: 0,
        bmSublinkSpeedAttr: [0xa4030, 0xa40b0],
    },
};

const USB_DESC_TYPE_STRING: u8 = 3;

const STR0_DESCRIPTOR: [u8; 4] = [4, USB_DESC_TYPE_STRING, 0x09, 0x04];

const STR_MANUFACTURER_DESCRIPTOR: [u8; 14] = [
    14,
    USB_DESC_TYPE_STRING,
    b'G',
    0,
    b'o',
    0,
    b'o',
    0,
    b'g',
    0,
    b'l',
    0,
    b'e',
    0,
];

const STR_INTERFACE_DESCRIPTOR: [u8; 18] = [
    18,
    USB_DESC_TYPE_STRING,
    b'f',
    0,
    b'a',
    0,
    b's',
    0,
    b't',
    0,
    b'b',
    0,
    b'o',
    0,
    b'o',
    0,
    b't',
    0,
];

const PRODUCT_DESCRIPTOR: [u8; 16] = [
    16,
    USB_DESC_TYPE_STRING,
    b'A',
    0,
    b'n',
    0,
    b'd',
    0,
    b'r',
    0,
    b'o',
    0,
    b'i',
    0,
    b'd',
    0,
];

#[derive(Debug)]
pub enum EfiUsbDeviceEvent {
    NoEvent,
    Connected,
    Disconnected,
    OutData(&'static [u8]),
}

#[allow(dead_code)]
impl EfiUsbDevice {
    fn build_string_descriptors(serial_number: &CStr16) -> [*const u8; 5] {
        let serial_number = serial_number.as_bytes();
        let length = serial_number.len();

        let mut serial_descriptor = [0u8; 32];
        serial_descriptor[0] = length as u8;
        serial_descriptor[1] = USB_DESC_TYPE_STRING;
        serial_descriptor[2..2 + length].copy_from_slice(serial_number);

        let serial_descriptor = Box::new(serial_descriptor);

        [
            STR0_DESCRIPTOR.as_ptr(),
            STR_MANUFACTURER_DESCRIPTOR.as_ptr(),
            PRODUCT_DESCRIPTOR.as_ptr(),
            Box::into_raw(serial_descriptor) as *const u8,
            STR_INTERFACE_DESCRIPTOR.as_ptr(),
        ]
    }

    pub fn start(&self, serial_number: &CStr16) -> Result {
        let string_descriptors = Self::build_string_descriptors(serial_number);

        unsafe {
            (self.0.start)(
                &DEVICE_DESCRIPTOR,
                CONFIG_DESCRIPTOR_TREES.as_ptr(),
                &DEVICE_QUALIFIER,
                &BINARY_OBJECT_STORE,
                5,
                string_descriptors.as_ptr(),
            )
        }
        .to_result()
    }

    pub fn send(&self, endpoint: u8, size: usize, buf: *mut u8) -> Result {
        unsafe { (self.0.send)(endpoint, size as u64, buf) }.to_result()
    }

    pub fn handle_event(&self) -> Result<EfiUsbDeviceEvent> {
        let mut event: UsbDeviceEvent = UsbDeviceEvent::UsbDeviceEventNoEvent;
        let mut event_size: u64 = 0;
        let mut event_data: UsbDeviceEventData = UsbDeviceEventData {
            state: UsbDeviceState::UsbDeviceStateDisconnected,
        };

        unsafe { (self.0.handle_event)(&mut event, &mut event_size, &mut event_data) }
            .to_result()?;

        match event {
            UsbDeviceEvent::UsbDeviceEventNoEvent => Ok(EfiUsbDeviceEvent::NoEvent),
            UsbDeviceEvent::UsbDeviceEventDeviceStateChange => {
                let state = unsafe { event_data.state };
                match state {
                    UsbDeviceState::UsbDeviceStateConnected => Ok(EfiUsbDeviceEvent::Connected),
                    UsbDeviceState::UsbDeviceStateDisconnected => {
                        Ok(EfiUsbDeviceEvent::Disconnected)
                    }
                }
            }
            UsbDeviceEvent::UsbDeviceEventTransferNotification => {
                let transfer_outcome = unsafe { event_data.transfer_outcome };
                match (transfer_outcome.endpoint_index, transfer_outcome.status) {
                    (ENDPOINT_OUT, UsbDeviceTransferStatus::UsbDeviceTransferStatusActive) => {
                        Ok(EfiUsbDeviceEvent::NoEvent)
                    }
                    (ENDPOINT_OUT, UsbDeviceTransferStatus::UsbDeviceTransferStatusCompleteOK) => {
                        let data = slice_from_raw_parts(
                            transfer_outcome.data,
                            transfer_outcome.bytes_completed as usize,
                        );
                        let test = unsafe { data.as_ref() }.unwrap();
                        Ok(EfiUsbDeviceEvent::OutData(test))
                    }
                    (ENDPOINT_OUT, UsbDeviceTransferStatus::UsbDeviceTransferStatusCancelled) => {
                        Ok(EfiUsbDeviceEvent::NoEvent)
                    }
                    (
                        ENDPOINT_OUT,
                        UsbDeviceTransferStatus::UsbDeviceTransferStatusCompleteError,
                    ) => Ok(EfiUsbDeviceEvent::NoEvent),
                    (ENDPOINT_IN, UsbDeviceTransferStatus::UsbDeviceTransferStatusActive) => {
                        Ok(EfiUsbDeviceEvent::NoEvent)
                    }
                    (ENDPOINT_IN, UsbDeviceTransferStatus::UsbDeviceTransferStatusCompleteOK) => {
                        Ok(EfiUsbDeviceEvent::NoEvent)
                    }
                    (ENDPOINT_IN, UsbDeviceTransferStatus::UsbDeviceTransferStatusCancelled) => {
                        Ok(EfiUsbDeviceEvent::NoEvent)
                    }
                    (
                        ENDPOINT_IN,
                        UsbDeviceTransferStatus::UsbDeviceTransferStatusCompleteError,
                    ) => Ok(EfiUsbDeviceEvent::NoEvent),
                    _ => Ok(EfiUsbDeviceEvent::NoEvent),
                }
            }
            UsbDeviceEvent::UsbDeviceEventOemEvent => Ok(EfiUsbDeviceEvent::NoEvent),
        }
    }

    pub fn allocate_transfer_buffer(&self, size: usize) -> Result<*mut u8> {
        let mut ptr = ptr::null_mut();
        unsafe { (self.0.allocate_transfer_buffer)(size as u64, &mut ptr) }
            .to_result_with_val(|| ptr)
    }

    pub fn free_transfer_buffer(&self, ptr: *mut u8) -> Result {
        unsafe { (self.0.free_transfer_buffer)(ptr) }.to_result()
    }

    pub fn set_endpoint_stall_state(&self, state: bool) -> Result {
        unsafe { (self.0.set_endpoint_stall_state)(0, state) }.to_result()
    }

    pub fn stop(&self) -> Result {
        unsafe { (self.0.stop)() }.to_result()
    }

    pub fn start_ex(&self, serial_number: &CStr16) -> Result {
        let string_descriptors = Self::build_string_descriptors(serial_number);

        let device_descriptor_set = UsbDeviceDescriptorSet {
            device_descriptor: &DEVICE_DESCRIPTOR,
            config_descriptor_trees: CONFIG_DESCRIPTOR_TREES.as_ptr(),
            superspeed_device_descriptor: &SUPERSPEED_DEVICE_DESCRIPTOR,
            superspeed_config_descriptor_trees: SUPERSPEED_CONFIG_DESCRIPTOR_TREES.as_ptr(),
            device_qualifier_descriptor: &DEVICE_QUALIFIER,
            binary_object_store: &BINARY_OBJECT_STORE,
            string_descriptor_count: 5,
            string_descriptors: string_descriptors.as_ptr(),
        };

        unsafe { (self.0.start_ex)(&device_descriptor_set) }.to_result()
    }
}
