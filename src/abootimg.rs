// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
// SPDX-License-Identifier: BSD-3-Clause

use core::ptr::slice_from_raw_parts_mut;
use log::info;
use uefi::boot::{self, MemoryType};
use uefi::proto::loaded_image::LoadedImage;
use uefi::{CStr16, Handle, Result};

use crate::EFI_FDT_TABLE;

use crate::initrd::LinuxInitrd;
use crate::FastbootBuffer;

const BOOT_MAGIC: &[u8; 8] = b"ANDROID!";

#[repr(C, packed)]
struct AndroidBootImageV2 {
    magic: [u8; 8],
    kernel_size: u32,
    kernel_addr: u32,
    ramdisk_size: u32,
    ramdisk_addr: u32,
    second_size: u32,
    second_addr: u32,
    tags_addr: u32,
    page_size: u32,
    header_version: u32,
    os_version: u32,
    name: [u8; 16],
    cmdline: [u8; 512],
    id: [u32; 8],
    extra_cmdline: [u8; 1024],
    recovery_dtbo_size: u32,
    recovery_dtbo_offset: u64,
    header_size: u32,
    dtb_size: u32,
    dtb_addr: u64,
}

pub(crate) fn is_bootimg(payload: &[u8]) -> bool {
    payload.starts_with(BOOT_MAGIC)
}

pub(crate) fn handle_bootimg(payload: &[u8]) -> Result<(Handle, Option<LinuxInitrd>)> {
    let aboot2: &AndroidBootImageV2 = unsafe { &*(payload.as_ptr().cast()) };

    let page_align = |offset: usize| {
        let mask = (aboot2.page_size - 1) as usize;
        (offset + (mask)) & !(mask)
    };

    let header_size = aboot2.header_size as usize;
    let kernel_offset = page_align(header_size);
    let kernel_size = aboot2.kernel_size as usize;
    let ramdisk_offset = page_align(kernel_offset + kernel_size);
    let ramdisk_size = aboot2.ramdisk_size as usize;
    let second_offset = page_align(ramdisk_offset + ramdisk_size);
    let second_size = aboot2.second_size as usize;
    let dtb_offset = page_align(second_offset + second_size);
    let dtb_size = aboot2.dtb_size as usize;

    info!(
        "loading kernel: {} byte from {}, ramdisk: {} bytes from {}, dtb: {} bytes from {}",
        kernel_size, kernel_offset, ramdisk_size, ramdisk_offset, dtb_size, dtb_offset
    );

    let mut kernel = FastbootBuffer::alloc(MemoryType::RUNTIME_SERVICES_CODE, kernel_size)?;
    kernel.write(&payload[kernel_offset..kernel_offset + kernel_size])?;

    let mut ramdisk = FastbootBuffer::alloc(MemoryType::BOOT_SERVICES_DATA, ramdisk_size)?;
    ramdisk.write(&payload[ramdisk_offset..ramdisk_offset + ramdisk_size])?;

    let mut dtb = FastbootBuffer::alloc(MemoryType::ACPI_RECLAIM, dtb_size)?;
    dtb.write(&payload[dtb_offset..dtb_offset + dtb_size])?;

    let cmdline_len = aboot2
        .cmdline
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(aboot2.cmdline.len());
    let cmdline = &aboot2.cmdline[..cmdline_len];
    let cmdline = core::str::from_utf8(cmdline).expect("Unable to parse command line");

    let mut cmdline_buf =
        boot::allocate_pool(MemoryType::BOOT_SERVICES_DATA, (cmdline.len() + 1) * 2)?.cast::<u16>();
    let cmdline_buf: &mut [u16] =
        unsafe { &mut *slice_from_raw_parts_mut(cmdline_buf.as_mut(), cmdline.len() + 1) };
    CStr16::from_str_with_buf(cmdline, cmdline_buf)
        .expect("Unable to convert command line to UCS-2");

    let handle = kernel.load_image()?;

    dtb.install_configuration_table(&EFI_FDT_TABLE)?;
    let mut loaded_image = boot::open_protocol_exclusive::<LoadedImage>(handle)?;
    unsafe {
        loaded_image.set_load_options(
            cmdline_buf.as_ptr() as *const u8,
            (cmdline.len() * 2).try_into().unwrap(),
        )
    };

    let initrd = LinuxInitrd::new(ramdisk);

    Ok((handle, Some(initrd)))
}
