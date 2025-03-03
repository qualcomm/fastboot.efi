// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
// SPDX-License-Identifier: BSD-3-Clause

use uefi::{boot::MemoryType, Handle, Result};

use crate::FastbootBuffer;

const PE_OFFSET: usize = 0x3c;
const PE_MAGIC: [u8; 4] = [b'P', b'E', 0, 0];
const PE_ARM64: u16 = 0xaa64;
const PE_PLUS: u16 = 0x020b;
const PE_SUBSYSTEM_EFI_APP: u16 = 10;

pub(crate) fn is_peimage(payload: &[u8]) -> bool {
    if payload.len() < PE_OFFSET + 4 || payload[0] != b'M' || payload[1] != b'Z' {
        return false;
    }

    let pe_offset: [u8; 4] = payload[PE_OFFSET..PE_OFFSET + 4].try_into().unwrap();
    let pe_offset = u32::from_le_bytes(pe_offset) as usize;

    if payload.len() < pe_offset + PE_MAGIC.len() + 70
        || payload[pe_offset..pe_offset + PE_MAGIC.len()] != PE_MAGIC
    {
        return false;
    }
    if payload[pe_offset..pe_offset + PE_MAGIC.len()] != PE_MAGIC {
        return false;
    }

    let coff_hdr = &payload[pe_offset + PE_MAGIC.len()..];
    let machine: [u8; 2] = coff_hdr[0..2].try_into().unwrap();
    let machine = u16::from_le_bytes(machine);

    if machine != PE_ARM64 {
        return false;
    }

    let opt_hdr_size: [u8; 2] = coff_hdr[16..18].try_into().unwrap();
    let opt_hdr_size = u16::from_le_bytes(opt_hdr_size);
    if opt_hdr_size < 88 {
        return false;
    }

    let opt_hdr = &coff_hdr[20..];
    let opt_magic: [u8; 2] = opt_hdr[0..2].try_into().unwrap();
    let opt_magic = u16::from_le_bytes(opt_magic);

    if opt_magic != PE_PLUS {
        return false;
    }

    let subsystem: [u8; 2] = opt_hdr[68..70].try_into().unwrap();
    let subsystem = u16::from_le_bytes(subsystem);

    if subsystem != PE_SUBSYSTEM_EFI_APP {
        return false;
    }

    true
}

pub(crate) fn handle_peimage(payload: &[u8]) -> Result<Handle> {
    let mut kernel = FastbootBuffer::alloc(MemoryType::RUNTIME_SERVICES_CODE, payload.len())?;
    kernel.write(payload)?;

    kernel.load_image()
}
