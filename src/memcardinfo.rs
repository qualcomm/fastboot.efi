// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
// SPDX-License-Identifier: BSD-3-Clause

use core::mem::MaybeUninit;
use uefi::{proto::unsafe_protocol, Result, Status, StatusExt};

use crate::proto::memcardinfo::{MemCardInfoProtocol, MemCardInfoStruct};

#[derive(Debug)]
#[repr(transparent)]
#[unsafe_protocol(MemCardInfoProtocol::GUID)]
pub struct MemCardInfo(MemCardInfoProtocol);

impl MemCardInfo {
    fn as_ffi_ptr(&self) -> *const MemCardInfoProtocol {
        let ptr: *const Self = self;
        ptr.cast::<MemCardInfoProtocol>()
    }

    pub fn get_card_info(&self) -> Result<MemCardInfoStruct> {
        let this = self.as_ffi_ptr();
        let mut info = MaybeUninit::<MemCardInfoStruct>::uninit();

        unsafe { (self.0.get_card_info)(this, info.as_mut_ptr()) }
            .to_result_with_val(|| unsafe { info.assume_init() })
    }

    #[allow(dead_code)]
    pub fn get_boot_unit(&self) -> Result<u32> {
        let this = self.as_ffi_ptr();
        let mut lu: u32 = 0;

        unsafe { (self.0.get_boot_unit)(this, &mut lu) }.to_result()?;

        Ok(lu)
    }

    #[allow(dead_code)]
    pub fn set_boot_unit(&self, lu: u32) -> Status {
        let this = self.as_ffi_ptr();

        unsafe { (self.0.set_boot_unit)(this, lu) }
    }
}
