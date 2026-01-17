use crate::error::set_error;
use crate::handle::{Elf, Elf_Scn, WriteState};
use crate::types::*;
use object::write::{Object as WriteObject, SectionKind};
use object::{Architecture, BinaryFormat, Endianness};
use std::ffi::c_void;
use std::io::Write;
use std::os::fd::FromRawFd;
use std::ptr;

type Elf64_Ehdr = object::elf::FileHeader64<Endianness>;

fn ensure_writer(elf: &mut Elf) -> bool {
    if elf.writer.is_some() {
        return true;
    }

    let arch = Architecture::X86_64;
    let endian = Endianness::Little;

    let obj = WriteObject::new(BinaryFormat::Elf, arch, endian);

    elf.writer = Some(WriteState {
        obj,
        sections: Vec::new(),
        section_data: Vec::new(),
        shstrtab_idx: None,
        ehdr64: None,
    });

    true
}

#[unsafe(no_mangle)]
pub extern "C" fn elf64_newehdr(elf: *mut Elf) -> *mut Elf64_Ehdr {
    if elf.is_null() {
        set_error("invalid argument");
        return ptr::null_mut();
    }

    let elf_ref = unsafe { &mut *elf };

    if !ensure_writer(elf_ref) {
        return ptr::null_mut();
    }

    let writer = elf_ref.writer.as_mut().unwrap();

    if writer.ehdr64.is_none() {
        writer.ehdr64 = Some(Box::new(unsafe { std::mem::zeroed() }));
    }

    writer.ehdr64.as_mut().unwrap().as_mut() as *mut Elf64_Ehdr
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_newscn(elf: *mut Elf) -> *mut Elf_Scn {
    if elf.is_null() {
        set_error("invalid argument");
        return ptr::null_mut();
    }

    let elf_ref = unsafe { &mut *elf };

    if !ensure_writer(elf_ref) {
        return ptr::null_mut();
    }

    let writer = elf_ref.writer.as_mut().unwrap();
    let section_idx = writer.sections.len();

    let section_id = writer
        .obj
        .add_section(Vec::new(), Vec::new(), SectionKind::Data);
    writer.sections.push(section_id);
    writer.section_data.push(Vec::new());

    let scn = Box::new(Elf_Scn::new(elf, section_idx));
    let scn_ptr = Box::into_raw(scn);
    elf_ref.section_handles.push(scn_ptr);
    scn_ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_newdata(scn: *mut Elf_Scn) -> *mut Elf_Data {
    if scn.is_null() {
        set_error("invalid argument");
        return ptr::null_mut();
    }

    let scn_ref = unsafe { &mut *scn };
    let elf = unsafe { &mut *scn_ref.elf };

    let data = Box::new(Elf_Data::default());
    let data_ptr = Box::into_raw(data);

    scn_ref.data_list_head = data_ptr;
    elf.data_handles.push(data_ptr);

    data_ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_setshstrndx(elf: *mut Elf, idx: usize) -> i32 {
    if elf.is_null() {
        return -1;
    }

    let elf_ref = unsafe { &mut *elf };

    if !ensure_writer(elf_ref) {
        return -1;
    }

    if let Some(writer) = elf_ref.writer.as_mut() {
        writer.shstrtab_idx = Some(idx);
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn elf_update(elf: *mut Elf, cmd: ElfCmd) -> i64 {
    if elf.is_null() {
        set_error("invalid argument");
        return -1;
    }

    let elf_ref = unsafe { &mut *elf };

    let writer = match elf_ref.writer.as_mut() {
        Some(w) => w,
        None => {
            set_error("no write state");
            return -1;
        }
    };

    for scn_ptr in elf_ref.section_handles.iter() {
        let scn = unsafe { &**scn_ptr };
        if !scn.data_list_head.is_null() {
            let data = unsafe { &*scn.data_list_head };
            if !data.d_buf.is_null() && data.d_size > 0 {
                let buf =
                    unsafe { std::slice::from_raw_parts(data.d_buf as *const u8, data.d_size) };
                if scn.index < writer.sections.len() {
                    writer.obj.set_section_data(
                        writer.sections[scn.index],
                        buf.to_vec(),
                        data.d_align as u64,
                    );
                }
            }
        }
    }

    match cmd {
        ELF_C_NULL => match writer.obj.write() {
            Ok(bytes) => bytes.len() as i64,
            Err(_) => {
                set_error("failed to compute layout");
                -1
            }
        },
        ELF_C_WRITE => {
            if elf_ref.fd < 0 {
                set_error("no file descriptor for write");
                return -1;
            }

            match writer.obj.write() {
                Ok(bytes) => {
                    let mut f = unsafe { std::fs::File::from_raw_fd(elf_ref.fd) };
                    if let Err(e) = f.write_all(&bytes) {
                        set_error(&format!("write failed: {e:?}"));
                        return -1;
                    };
                    bytes.len() as i64
                }
                Err(_) => {
                    set_error("failed to write ELF");
                    -1
                }
            }
        }
        _ => {
            set_error("unsupported command");
            -1
        }
    }
}
