use crate::error::set_error;
use crate::handle::{Elf, Elf_Scn};
use crate::types::*;
use object::Endianness;
use object::NativeEndian;
use std::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn gelf_getclass(elf: *mut Elf) -> i32 {
    if elf.is_null() {
        return ELFCLASSNONE as i32;
    }

    let elf_ref = unsafe { &mut *elf };
    if !elf_ref.ensure_parsed() {
        return ELFCLASSNONE as i32;
    }

    elf_ref
        .with_parsed(|p| {
            if p.is_elf32() {
                ELFCLASS32 as i32
            } else {
                ELFCLASS64 as i32
            }
        })
        .unwrap_or(ELFCLASSNONE as i32)
}

#[unsafe(no_mangle)]
pub extern "C" fn gelf_getehdr(elf: *mut Elf, dst: *mut GElf_Ehdr) -> *mut GElf_Ehdr {
    if elf.is_null() || dst.is_null() {
        set_error("invalid argument");
        return ptr::null_mut();
    }

    let elf_ref = unsafe { &mut *elf };
    if !elf_ref.ensure_parsed() {
        return ptr::null_mut();
    }

    match elf_ref.with_parsed(|p| p.get_ehdr()) {
        Some(ehdr) => {
            unsafe { *dst = ehdr };
            dst
        }
        None => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gelf_getshdr(scn: *mut Elf_Scn, dst: *mut GElf_Shdr) -> *mut GElf_Shdr {
    if scn.is_null() || dst.is_null() {
        set_error("invalid argument");
        return ptr::null_mut();
    }

    let scn_ref = unsafe { &*scn };
    let elf = unsafe { &mut *scn_ref.elf };

    if !elf.ensure_parsed() {
        return ptr::null_mut();
    }

    match elf.with_parsed(|p| p.get_shdr(scn_ref.index)).flatten() {
        Some(shdr) => {
            unsafe { *dst = shdr };
            dst
        }
        None => {
            set_error("failed to get section header");
            ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gelf_getphdr(elf: *mut Elf, index: i32, dst: *mut GElf_Phdr) -> *mut GElf_Phdr {
    if elf.is_null() || dst.is_null() || index < 0 {
        set_error("invalid argument");
        return ptr::null_mut();
    }

    let elf_ref = unsafe { &mut *elf };
    if !elf_ref.ensure_parsed() {
        return ptr::null_mut();
    }

    match elf_ref
        .with_parsed(|p| p.get_phdr(index as usize))
        .flatten()
    {
        Some(phdr) => {
            unsafe { *dst = phdr };
            dst
        }
        None => {
            set_error("failed to get program header");
            ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gelf_getsym(data: *mut Elf_Data, ndx: i32, dst: *mut GElf_Sym) -> *mut GElf_Sym {
    if data.is_null() || dst.is_null() || ndx < 0 {
        set_error("invalid argument");
        return ptr::null_mut();
    }

    let data_ref = unsafe { &*data };
    let buf = unsafe { std::slice::from_raw_parts(data_ref.d_buf as *const u8, data_ref.d_size) };

    let sym_size = std::mem::size_of::<object::elf::Sym64<Endianness>>();
    let offset = (ndx as usize) * sym_size;

    if offset + sym_size > buf.len() {
        set_error("symbol index out of range");
        return ptr::null_mut();
    }

    let sym: &object::elf::Sym64<Endianness> =
        match object::pod::from_bytes(&buf[offset..offset + sym_size]) {
            Ok((s, _)) => s,
            Err(_) => return ptr::null_mut(),
        };

    let endian = Endianness::Little;

    unsafe {
        (*dst).st_name.set(NativeEndian, sym.st_name.get(endian));
        (*dst).st_info = sym.st_info;
        (*dst).st_other = sym.st_other;
        (*dst).st_shndx.set(NativeEndian, sym.st_shndx.get(endian));
        (*dst).st_value.set(NativeEndian, sym.st_value.get(endian));
        (*dst).st_size.set(NativeEndian, sym.st_size.get(endian));
    }

    dst
}

#[unsafe(no_mangle)]
pub extern "C" fn gelf_getversym(
    data: *mut Elf_Data,
    ndx: i32,
    dst: *mut GElf_Versym,
) -> *mut GElf_Versym {
    if data.is_null() || dst.is_null() || ndx < 0 {
        return ptr::null_mut();
    }

    let data_ref = unsafe { &*data };
    let buf = unsafe { std::slice::from_raw_parts(data_ref.d_buf as *const u8, data_ref.d_size) };

    let versym_size = std::mem::size_of::<GElf_Versym>();
    let offset = (ndx as usize) * versym_size;

    if offset + versym_size > buf.len() {
        return ptr::null_mut();
    }

    let versym = u16::from_le_bytes([buf[offset], buf[offset + 1]]);
    unsafe { (*dst).0.set(NativeEndian, versym) };
    dst
}

#[unsafe(no_mangle)]
pub extern "C" fn gelf_getverdef(
    data: *mut Elf_Data,
    offset: i32,
    dst: *mut GElf_Verdef,
) -> *mut GElf_Verdef {
    if data.is_null() || dst.is_null() || offset < 0 {
        return ptr::null_mut();
    }

    let data_ref = unsafe { &*data };
    let buf = unsafe { std::slice::from_raw_parts(data_ref.d_buf as *const u8, data_ref.d_size) };

    let off = offset as usize;
    if off + 20 > buf.len() {
        return ptr::null_mut();
    }

    let endian = Endianness::Little;
    let verdef: &object::elf::Verdef<Endianness> = match object::pod::from_bytes(&buf[off..]) {
        Ok((v, _)) => v,
        Err(_) => return ptr::null_mut(),
    };

    unsafe {
        (*dst)
            .vd_version
            .set(NativeEndian, verdef.vd_version.get(endian));
        (*dst)
            .vd_flags
            .set(NativeEndian, verdef.vd_flags.get(endian));
        (*dst).vd_ndx.set(NativeEndian, verdef.vd_ndx.get(endian));
        (*dst).vd_cnt.set(NativeEndian, verdef.vd_cnt.get(endian));
        (*dst).vd_hash.set(NativeEndian, verdef.vd_hash.get(endian));
        (*dst).vd_aux.set(NativeEndian, verdef.vd_aux.get(endian));
        (*dst).vd_next.set(NativeEndian, verdef.vd_next.get(endian));
    }

    dst
}

#[unsafe(no_mangle)]
pub extern "C" fn gelf_getverdaux(
    data: *mut Elf_Data,
    offset: i32,
    dst: *mut GElf_Verdaux,
) -> *mut GElf_Verdaux {
    if data.is_null() || dst.is_null() || offset < 0 {
        return ptr::null_mut();
    }

    let data_ref = unsafe { &*data };
    let buf = unsafe { std::slice::from_raw_parts(data_ref.d_buf as *const u8, data_ref.d_size) };

    let off = offset as usize;
    if off + 8 > buf.len() {
        return ptr::null_mut();
    }

    let endian = Endianness::Little;
    let verdaux: &object::elf::Verdaux<Endianness> = match object::pod::from_bytes(&buf[off..]) {
        Ok((v, _)) => v,
        Err(_) => return ptr::null_mut(),
    };

    unsafe {
        (*dst)
            .vda_name
            .set(NativeEndian, verdaux.vda_name.get(endian));
        (*dst)
            .vda_next
            .set(NativeEndian, verdaux.vda_next.get(endian));
    }

    dst
}

#[unsafe(no_mangle)]
pub extern "C" fn gelf_getnote(
    data: *mut Elf_Data,
    offset: usize,
    nhdr: *mut GElf_Nhdr,
    name_offset: *mut usize,
    desc_offset: *mut usize,
) -> usize {
    if data.is_null() || nhdr.is_null() {
        return 0;
    }

    let data_ref = unsafe { &*data };
    let buf = unsafe { std::slice::from_raw_parts(data_ref.d_buf as *const u8, data_ref.d_size) };

    if offset >= buf.len() {
        return 0;
    }

    let remaining = &buf[offset..];
    if remaining.len() < 12 {
        return 0;
    }

    let n_namesz = u32::from_le_bytes([remaining[0], remaining[1], remaining[2], remaining[3]]);
    let n_descsz = u32::from_le_bytes([remaining[4], remaining[5], remaining[6], remaining[7]]);
    let n_type = u32::from_le_bytes([remaining[8], remaining[9], remaining[10], remaining[11]]);

    unsafe {
        (*nhdr).n_namesz.set(NativeEndian, n_namesz);
        (*nhdr).n_descsz.set(NativeEndian, n_descsz);
        (*nhdr).n_type.set(NativeEndian, n_type);
    }

    let align = 4usize;
    let name_off = offset + 12;
    let name_aligned_size = (n_namesz as usize + align - 1) & !(align - 1);
    let desc_off = name_off + name_aligned_size;
    let desc_aligned_size = (n_descsz as usize + align - 1) & !(align - 1);
    let next_offset = desc_off + desc_aligned_size;

    if !name_offset.is_null() {
        unsafe { *name_offset = name_off };
    }
    if !desc_offset.is_null() {
        unsafe { *desc_offset = desc_off };
    }

    if next_offset > buf.len() {
        0
    } else {
        next_offset
    }
}
