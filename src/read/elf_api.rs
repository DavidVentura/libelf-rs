use crate::error::set_error;
use crate::handle::{Elf, Elf_Scn};
use crate::types::*;
use object::FileKind;
use object::NativeEndian;
use std::ffi::c_void;
use std::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn elf_version(ver: u32) -> u32 {
    if ver == EV_NONE.into() {
        EV_CURRENT.into()
    } else if ver > EV_CURRENT.into() {
        EV_NONE.into()
    } else {
        EV_CURRENT.into()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_begin(fd: i32, cmd: ElfCmd, _ref_elf: *mut Elf) -> *mut Elf {
    if fd < 0 {
        set_error("invalid file descriptor");
        return ptr::null_mut();
    }

    if cmd == ELF_C_WRITE {
        let elf = Box::new(Elf {
            fd,
            cmd,
            data: ptr::null(),
            data_len: 0,
            owned_data: None,
            mmap: None,
            parsed: None,
            section_handles: Vec::new(),
            data_handles: Vec::new(),
            section_data_cache: Vec::new(),
            writer: None,
        });
        return Box::into_raw(elf);
    }

    let mmap = match memmap2::MmapOptions::new().map_raw(fd) {
        Ok(m) => m,
        Err(_) => {
            set_error("mmap failed");
            return ptr::null_mut();
        }
    };

    let data_len = mmap.len();
    let data_ptr = mmap.as_ptr();

    let elf = Box::new(Elf {
        fd,
        cmd,
        data: data_ptr,
        data_len,
        owned_data: None,
        mmap: Some(mmap),
        parsed: None,
        section_handles: Vec::new(),
        data_handles: Vec::new(),
        section_data_cache: Vec::new(),
        writer: None,
    });

    Box::into_raw(elf)
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_memory(image: *mut i8, size: usize) -> *mut Elf {
    if image.is_null() || size == 0 {
        set_error("invalid memory image");
        return ptr::null_mut();
    }

    let owned_data = unsafe { std::slice::from_raw_parts(image as *const u8, size).to_vec() };

    let mut elf = Box::new(Elf {
        fd: -1,
        cmd: ELF_C_READ,
        data: ptr::null(),
        data_len: size,
        owned_data: Some(owned_data),
        mmap: None,
        parsed: None,
        section_handles: Vec::new(),
        data_handles: Vec::new(),
        section_data_cache: Vec::new(),
        writer: None,
    });

    elf.data = elf.owned_data.as_ref().unwrap().as_ptr();
    Box::into_raw(elf)
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_end(elf: *mut Elf) -> i32 {
    if elf.is_null() {
        return 0;
    }
    unsafe { drop(Box::from_raw(elf)) };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_kind(elf: *mut Elf) -> ElfKind {
    if elf.is_null() {
        return ELF_K_NONE;
    }

    let elf = unsafe { &*elf };
    let data = unsafe { std::slice::from_raw_parts(elf.data, elf.data_len) };

    match FileKind::parse(data) {
        Ok(FileKind::Elf32) | Ok(FileKind::Elf64) => ELF_K_ELF,
        Ok(FileKind::Archive) => ELF_K_AR,
        _ => ELF_K_NONE,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_nextscn(elf: *mut Elf, scn: *mut Elf_Scn) -> *mut Elf_Scn {
    if elf.is_null() {
        return ptr::null_mut();
    }

    let elf_ref = unsafe { &mut *elf };
    if !elf_ref.ensure_parsed() {
        return ptr::null_mut();
    }

    let next_idx = if scn.is_null() {
        1 // skip section 0 (null section)
    } else {
        unsafe { (*scn).index + 1 }
    };

    let section_count = elf_ref.with_parsed(|p| p.section_count()).unwrap_or(0);
    if next_idx >= section_count {
        return ptr::null_mut();
    }

    let scn_box = Box::new(Elf_Scn::new(elf, next_idx));
    let scn_ptr = Box::into_raw(scn_box);
    elf_ref.section_handles.push(scn_ptr);
    scn_ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_getscn(elf: *mut Elf, index: usize) -> *mut Elf_Scn {
    if elf.is_null() {
        return ptr::null_mut();
    }

    let elf_ref = unsafe { &mut *elf };
    if !elf_ref.ensure_parsed() {
        return ptr::null_mut();
    }

    let section_count = elf_ref.with_parsed(|p| p.section_count()).unwrap_or(0);
    if index >= section_count {
        set_error("invalid section index");
        return ptr::null_mut();
    }

    let scn_box = Box::new(Elf_Scn::new(elf, index));
    let scn_ptr = Box::into_raw(scn_box);
    elf_ref.section_handles.push(scn_ptr);
    scn_ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_ndxscn(scn: *mut Elf_Scn) -> usize {
    if scn.is_null() {
        return 0;
    }
    unsafe { (*scn).index }
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_getshdrstrndx(elf: *mut Elf, dst: *mut usize) -> i32 {
    if elf.is_null() || dst.is_null() {
        set_error("invalid argument");
        return -1;
    }

    let elf_ref = unsafe { &mut *elf };
    if !elf_ref.ensure_parsed() {
        return -1;
    }

    match elf_ref.with_parsed(|p| p.shstrndx()) {
        Some(idx) => {
            unsafe { *dst = idx };
            0
        }
        None => {
            set_error("failed to get shstrndx");
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_getphdrnum(elf: *mut Elf, dst: *mut usize) -> i32 {
    if elf.is_null() || dst.is_null() {
        set_error("invalid argument");
        return -1;
    }

    let elf_ref = unsafe { &mut *elf };
    if !elf_ref.ensure_parsed() {
        return -1;
    }

    match elf_ref.with_parsed(|p| p.program_header_count()) {
        Some(count) => {
            unsafe { *dst = count };
            0
        }
        None => {
            set_error("failed to get phdrnum");
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data {
    if scn.is_null() {
        return ptr::null_mut();
    }

    if !data.is_null() {
        return ptr::null_mut();
    }

    let scn_ref = unsafe { &*scn };
    let elf = unsafe { &mut *scn_ref.elf };

    if !elf.ensure_parsed() {
        return ptr::null_mut();
    }

    let section_data = elf.with_parsed(|p| p.section_data(scn_ref.index).map(|d| d.to_vec()));

    match section_data {
        Some(Some(data_vec)) => {
            let idx = elf.section_data_cache.len();
            elf.section_data_cache.push(data_vec);

            let shdr = elf.with_parsed(|p| p.get_shdr(scn_ref.index)).flatten();
            let align = shdr
                .map(|s| s.sh_addralign.get(NativeEndian) as usize)
                .unwrap_or(1);

            let elf_data = Box::new(Elf_Data {
                d_buf: elf.section_data_cache[idx].as_mut_ptr() as *mut c_void,
                d_size: elf.section_data_cache[idx].len(),
                d_type: ELF_T_BYTE,
                d_version: EV_CURRENT.into(),
                d_off: 0,
                d_align: align.max(1),
            });

            let data_ptr = Box::into_raw(elf_data);
            elf.data_handles.push(data_ptr);
            data_ptr
        }
        _ => {
            set_error("failed to get section data");
            ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_rawdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data {
    elf_getdata(scn, data)
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_strptr(elf: *mut Elf, section: usize, offset: usize) -> *const i8 {
    if elf.is_null() {
        return ptr::null();
    }

    let elf_ref = unsafe { &mut *elf };
    if !elf_ref.ensure_parsed() {
        return ptr::null();
    }

    let shdr = match elf_ref.with_parsed(|p| p.get_shdr(section)).flatten() {
        Some(s) => s,
        None => return ptr::null(),
    };

    if shdr.sh_type.get(NativeEndian) != SHT_STRTAB {
        set_error("not a string table section");
        return ptr::null();
    }

    let section_offset = shdr.sh_offset.get(NativeEndian) as usize;
    let section_size = shdr.sh_size.get(NativeEndian) as usize;

    if offset >= section_size {
        set_error("offset out of bounds");
        return ptr::null();
    }

    unsafe { elf_ref.data.add(section_offset + offset) as *const i8 }
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_flagdata(_data: *mut Elf_Data, _cmd: ElfCmd, flags: u32) -> u32 {
    flags
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_flagshdr(scn: *mut Elf_Scn, _cmd: ElfCmd, flags: u32) -> u32 {
    if scn.is_null() {
        return 0;
    }
    unsafe { (*scn).flags |= flags };
    flags
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_getshdrnum(elf: *mut Elf, dst: *mut usize) -> i32 {
    if elf.is_null() || dst.is_null() {
        set_error("invalid argument");
        return -1;
    }

    let elf_ref = unsafe { &mut *elf };
    if !elf_ref.ensure_parsed() {
        return -1;
    }

    match elf_ref.with_parsed(|p| p.section_count()) {
        Some(count) => {
            unsafe { *dst = count };
            0
        }
        None => {
            set_error("failed to get shdrnum");
            -1
        }
    }
}

pub type Elf64_Ehdr = object::elf::FileHeader64<object::Endianness>;
pub type Elf64_Shdr = object::elf::SectionHeader64<object::Endianness>;

#[unsafe(no_mangle)]
pub extern "C" fn elf64_getehdr(elf: *mut Elf) -> *mut Elf64_Ehdr {
    if elf.is_null() {
        return ptr::null_mut();
    }

    let elf_ref = unsafe { &*elf };
    elf_ref.data as *mut Elf64_Ehdr
}

#[unsafe(no_mangle)]
pub extern "C" fn elf64_getshdr(scn: *mut Elf_Scn) -> *mut Elf64_Shdr {
    if scn.is_null() {
        return ptr::null_mut();
    }

    let scn_ref = unsafe { &*scn };
    let elf = unsafe { &*scn_ref.elf };

    let ehdr = elf.data as *const Elf64_Ehdr;
    let shoff = unsafe { (*ehdr).e_shoff.get(object::Endianness::Little) } as usize;
    let shentsize = unsafe { (*ehdr).e_shentsize.get(object::Endianness::Little) } as usize;

    let shdr_offset = shoff + scn_ref.index * shentsize;
    unsafe { elf.data.add(shdr_offset) as *mut Elf64_Shdr }
}
