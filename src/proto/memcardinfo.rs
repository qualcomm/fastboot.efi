// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
// SPDX-License-Identifier: BSD-3-Clause

use uefi::{guid, Guid, Status};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemCardInfoStruct {
    pub manufacturer_id: u16,
    pub oem_id: u16,
    pub manufacturer_date: [u8; 8],
    pub serial_number: [u8; 252],
    pub serial_number_len: u32,
    pub inquiry_str: [u8; 29],
    pub rpmb_size: u32,
    pub reliable_write_count: u32,
    pub card_type: [u8; 4],
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemCardInfoProtocol {
    pub revision: u64,

    pub get_card_info: unsafe extern "efiapi" fn(
        this: *const MemCardInfoProtocol,
        card_info: *mut MemCardInfoStruct,
    ) -> Status,
    pub get_boot_unit:
        unsafe extern "efiapi" fn(this: *const MemCardInfoProtocol, lu: *mut u32) -> Status,
    pub set_boot_unit:
        unsafe extern "efiapi" fn(this: *const MemCardInfoProtocol, lu: u32) -> Status,
}

impl MemCardInfoProtocol {
    pub const GUID: Guid = guid!("85c1f7d2-bce6-4f31-8f4d-d37e03d05eaa");
}
