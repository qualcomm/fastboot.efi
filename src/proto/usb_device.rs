// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
// SPDX-License-Identifier: BSD-3-Clause

use uefi::{guid, Guid, Status};

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(crate) struct UsbDeviceDescriptor {
    pub(crate) bLength: u8,
    pub(crate) bDescriptorType: u8,
    pub(crate) bcdUSB: u16,
    pub(crate) bDeviceClass: u8,
    pub(crate) bDeviceSubClass: u8,
    pub(crate) bDeviceProtocol: u8,
    pub(crate) bMaxPacketSize0: u8,
    pub(crate) idVendor: u16,
    pub(crate) idProduct: u16,
    pub(crate) bcdDevice: u16,
    pub(crate) iManufacturer: u8,
    pub(crate) iProduct: u8,
    pub(crate) iSerialNumber: u8,
    pub(crate) bNumConfigurations: u8,
}
pub(crate) const USB_DEVICE_DESCRIPTOR_LEN: u8 = 18;
pub(crate) const USB_DEVICE_DESCRIPTOR_TYPE: u8 = 1;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(crate) struct UsbDeviceQualifierDescriptor {
    pub(crate) bLength: u8,
    pub(crate) bDescriptorType: u8,
    pub(crate) bcdUSB: u16,
    pub(crate) bDeviceClass: u8,
    pub(crate) bDeviceSubClass: u8,
    pub(crate) bDeviceProtocol: u8,
    pub(crate) bMaxPacketSize0: u8,
    pub(crate) bNumConfigurations: u8,
    pub(crate) bReserved: u8,
}
pub(crate) const USB_DEVICE_QUALIFIER_DESCRIPTOR_LEN: u8 = 10;
pub(crate) const USB_DEVICE_QUALIFIER_DESCRIPTOR_TYPE: u8 = 0x6;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(crate) struct UsbConfigDescriptor {
    pub(crate) bLength: u8,
    pub(crate) bDescriptorType: u8,
    pub(crate) wTotalLength: u16,
    pub(crate) bNumInterfaces: u8,
    pub(crate) bConfigurationValue: u8,
    pub(crate) iConfiguration: u8,
    pub(crate) bmAttributes: u8,
    pub(crate) bMaxPower: u8,
}
pub(crate) const USB_CONFIG_DESCRIPTOR_LEN: u8 = 9;
pub(crate) const USB_CONFIG_DESCRIPTOR_TYPE: u8 = 2;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(crate) struct UsbInterfaceDescriptor {
    pub(crate) bLength: u8,
    pub(crate) bDescriptorType: u8,
    pub(crate) bInterfaceNumber: u8,
    pub(crate) bAlternateSetting: u8,
    pub(crate) bNumEndpoints: u8,
    pub(crate) bInterfaceClass: u8,
    pub(crate) bInterfaceSubClass: u8,
    pub(crate) bInterfaceProtocol: u8,
    pub(crate) iInterface: u8,
}
pub(crate) const USB_INTERFACE_DESCRIPTOR_LEN: u8 = 9;
pub(crate) const USB_INTERFACE_DESCRIPTOR_TYPE: u8 = 4;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(crate) struct UsbEndpointDescriptor {
    pub(crate) bLength: u8,
    pub(crate) bDescriptorType: u8,
    pub(crate) bEndpointAddress: u8,
    pub(crate) bmAttributes: u8,
    pub(crate) wMaxPacketSize: u16,
    pub(crate) bInterval: u8,
}
pub(crate) const USB_ENDPOINT_DESCRIPTOR_LEN: u8 = 7;
pub(crate) const USB_ENDPOINT_DESCRIPTOR_TYPE: u8 = 5;
pub(crate) const USB_ENDPOINT_TYPE_BULK: u8 = 2;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(crate) struct UsbSuperSpeedCompanionDescriptor {
    pub(crate) bLength: u8,
    pub(crate) bDescriptorType: u8,
    pub(crate) bMaxBurst: u8,
    pub(crate) bmAttributes: u8,
    pub(crate) wBytesPerInterval: u16,
}
pub(crate) const USB_SUPER_SPEED_COMPANION_DESCRIPTOR_LEN: u8 = 6;
pub(crate) const USB_SUPER_SPEED_COMPANION_DESCRIPTOR_TYPE: u8 = 0x30;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(crate) struct UsbBinaryObjectStoreDescriptor {
    pub(crate) bLength: u8,
    pub(crate) bDescriptorType: u8,
    pub(crate) wTotalLength: u16,
    pub(crate) bNumDeviceCaps: u8,
}
pub(crate) const USB_BINARY_OBJECT_STORE_DESCRIPTOR_LEN: u8 = 5;
pub(crate) const USB_BINARY_OBJECT_STORE_DESCRIPTOR_TYPE: u8 = 0xf;

pub(crate) const USB_DEVICE_CAPABLITY_DESCRIPTOR_TYPE: u8 = 0x10;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(crate) struct UsbDeviceCapablityUsb20ExtensionDescriptor {
    pub(crate) bLength: u8,
    pub(crate) bDescriptorType: u8,
    pub(crate) bDevCapabilityType: u8,
    pub(crate) bmAttributes: u32,
}
pub(crate) const USB_DEVICE_CAPABLITY_USB20_EXTENSION_DESCRIPTOR_LEN: u8 = 7;
pub(crate) const USB_DEVICE_CAPABLITY_USB20_EXTENSION_DESCRIPTOR_CAPABILITY_TYPE: u8 = 0x2;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(crate) struct UsbDeviceCapablitySuperSpeedDescriptor {
    pub(crate) bLength: u8,
    pub(crate) bDescriptorType: u8,
    pub(crate) bDevCapabilityType: u8,
    pub(crate) bmAttributes: u8,
    pub(crate) wSpeedsSupported: u16,
    pub(crate) bFunctionalitySupport: u8,
    pub(crate) bU1DevExitLat: u8,
    pub(crate) wU2DevExitLat: u16,
}
pub(crate) const USB_DEVICE_CAPABLITY_SUPER_SPEED_DESCRIPTOR_LEN: u8 = 10;
pub(crate) const USB_DEVICE_CAPABLITY_SUPER_SPEED_DESCRIPTOR_CAPABILTY_TYPE: u8 = 0x3;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(crate) struct UsbDeviceCapablitySuperSpeedPlusDescriptor {
    pub(crate) bLength: u8,
    pub(crate) bDescriptorType: u8,
    pub(crate) bDevCapabilityType: u8,
    pub(crate) bReserved: u8,
    pub(crate) bmAttributes: u32,
    pub(crate) wFunctionalitySupport: u16,
    pub(crate) wReserved: u16,
    pub(crate) bmSublinkSpeedAttr: [u32; 2],
}
pub(crate) const USB_DEVICE_CAPABLITY_SUPER_SPEED_PLUS_DESCRIPTOR_LEN: u8 = 20;
pub(crate) const USB_DEVICE_CAPABLITY_SUPER_SPEED_PLUS_DESCRIPTOR_CAPABILITY_TYPE: u8 = 0xa;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) enum UsbDeviceEvent {
    NoEvent,
    DeviceStateChange,
    TransferNotification,
    OemEvent,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) enum UsbDeviceState {
    Connected,
    Disconnected,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) enum UsbDeviceTransferStatus {
    Active,
    CompleteOK,
    Cancelled,
    CompleteError,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct UsbDeviceTransferOutcome {
    pub(crate) status: UsbDeviceTransferStatus,
    pub(crate) endpoint_index: u8,
    pub(crate) bytes_completed: u64,
    pub(crate) data: *mut u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct UsbDeviceOemData {
    pub(crate) guid: Guid,
    pub(crate) version: u64,
    pub(crate) size: u64,
    pub(crate) context: *const u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) union UsbDeviceEventData {
    pub(crate) state: UsbDeviceState,
    pub(crate) transfer_outcome: UsbDeviceTransferOutcome,
    pub(crate) oem_data: UsbDeviceOemData,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct ConfigDescriptorTree {
    pub(crate) config_descriptor: UsbConfigDescriptor,
    pub(crate) interface_descriptor: UsbInterfaceDescriptor,
    pub(crate) endpoint0_descriptor: UsbEndpointDescriptor,
    pub(crate) endpoint1_descriptor: UsbEndpointDescriptor,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct BinaryObjectStore {
    pub(crate) descriptor: UsbBinaryObjectStoreDescriptor,
    pub(crate) usb_20_descriptor: UsbDeviceCapablityUsb20ExtensionDescriptor,
    pub(crate) superspeed_capablity_descriptor: UsbDeviceCapablitySuperSpeedDescriptor,
    pub(crate) superspeed_plus_capability_descriptor: UsbDeviceCapablitySuperSpeedPlusDescriptor,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct SuperSpeedConfigDescriptorTree {
    pub(crate) config_descriptor: UsbConfigDescriptor,
    pub(crate) interface_descriptor: UsbInterfaceDescriptor,
    pub(crate) endpoint0_descriptor: UsbEndpointDescriptor,
    pub(crate) superspeed_ednpoint0_compaion_descriptor: UsbSuperSpeedCompanionDescriptor,
    pub(crate) endpoint1_descriptor: UsbEndpointDescriptor,
    pub(crate) superspeed_ednpoint1_compaion_descriptor: UsbSuperSpeedCompanionDescriptor,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct UsbDeviceDescriptorSet {
    pub(crate) device_descriptor: *const UsbDeviceDescriptor,
    pub(crate) config_descriptor_trees: *const *const ConfigDescriptorTree,
    pub(crate) superspeed_device_descriptor: *const UsbDeviceDescriptor,
    pub(crate) superspeed_config_descriptor_trees: *const *const SuperSpeedConfigDescriptorTree,
    pub(crate) device_qualifier_descriptor: *const UsbDeviceQualifierDescriptor,
    pub(crate) binary_object_store: *const BinaryObjectStore,
    pub(crate) string_descriptor_count: u8,
    pub(crate) string_descriptors: *const *const u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct EfiUsbDeviceProtocol {
    pub(crate) revision: u64,

    pub(crate) start: unsafe extern "efiapi" fn(
        device_descriptor: *const UsbDeviceDescriptor,
        descriptors: *const *const ConfigDescriptorTree,
        qualifier_descriptor: *const UsbDeviceQualifierDescriptor,
        binary_object_store: *const BinaryObjectStore,
        string_descriptor_count: u8,
        string_descriptors: *const *const u8,
    ) -> Status,
    pub(crate) send: unsafe extern "efiapi" fn(endpoint: u8, size: u64, *mut u8) -> Status,
    pub(crate) handle_event: unsafe extern "efiapi" fn(
        event: *mut UsbDeviceEvent,
        size: *mut u64,
        data: *mut UsbDeviceEventData,
    ) -> Status,
    pub(crate) allocate_transfer_buffer: unsafe extern "efiapi" fn(size: u64, ptr: *mut *mut u8) -> Status,
    pub(crate) free_transfer_buffer: unsafe extern "efiapi" fn(ptr: *mut u8) -> Status,
    pub(crate) stop: unsafe extern "efiapi" fn() -> Status,
    pub(crate) abort_xfer: unsafe extern "efiapi" fn() -> Status,
    pub(crate) set_endpoint_stall_state: unsafe extern "efiapi" fn(ep_idx: u8, state: bool) -> Status,
    pub(crate) start_ex: unsafe extern "efiapi" fn(desc: *const UsbDeviceDescriptorSet) -> Status,
}

impl EfiUsbDeviceProtocol {
    pub const GUID: Guid = guid!("d9d9ce48-44b8-4f49-8e3e-2a3b927dc6c1");
}
