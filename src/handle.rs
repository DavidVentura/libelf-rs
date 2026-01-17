use crate::error::set_error;
use crate::types::*;
use object::read::elf::{ElfFile32, ElfFile64, FileHeader, ProgramHeader, SectionHeader};
use object::write::Object as WriteObject;
use object::{Endianness, FileKind, SectionIndex};

pub enum ParsedElf<'a> {
    Elf32(ElfFile32<'a, Endianness>),
    Elf64(ElfFile64<'a, Endianness>),
}

impl<'a> ParsedElf<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, &'static str> {
        match FileKind::parse(data) {
            Ok(FileKind::Elf32) => ElfFile32::parse(data)
                .map(ParsedElf::Elf32)
                .map_err(|_| "failed to parse ELF32"),
            Ok(FileKind::Elf64) => ElfFile64::parse(data)
                .map(ParsedElf::Elf64)
                .map_err(|_| "failed to parse ELF64"),
            Ok(_) => Err("not an ELF file"),
            Err(_) => Err("failed to determine file kind"),
        }
    }

    pub fn is_elf32(&self) -> bool {
        matches!(self, ParsedElf::Elf32(_))
    }

    pub fn is_elf64(&self) -> bool {
        matches!(self, ParsedElf::Elf64(_))
    }

    pub fn endianness(&self) -> Endianness {
        match self {
            ParsedElf::Elf32(e) => e.endian(),
            ParsedElf::Elf64(e) => e.endian(),
        }
    }

    pub fn section_count(&self) -> usize {
        match self {
            ParsedElf::Elf32(e) => e.elf_section_table().len(),
            ParsedElf::Elf64(e) => e.elf_section_table().len(),
        }
    }

    pub fn program_header_count(&self) -> usize {
        match self {
            ParsedElf::Elf32(e) => e.elf_program_headers().len(),
            ParsedElf::Elf64(e) => e.elf_program_headers().len(),
        }
    }

    pub fn shstrndx(&self) -> usize {
        match self {
            ParsedElf::Elf32(e) => e.elf_header().e_shstrndx(e.endian()) as usize,
            ParsedElf::Elf64(e) => e.elf_header().e_shstrndx(e.endian()) as usize,
        }
    }

    pub fn get_ehdr(&self) -> GElf_Ehdr {
        let mut result: GElf_Ehdr = unsafe { std::mem::zeroed() };
        match self {
            ParsedElf::Elf32(e) => {
                let h = e.elf_header();
                let endian = e.endian();
                result.e_ident = *h.e_ident();
                result.e_type.set(NativeEndian, h.e_type(endian));
                result.e_machine.set(NativeEndian, h.e_machine(endian));
                result.e_version.set(NativeEndian, h.e_version(endian));
                result.e_entry.set(NativeEndian, h.e_entry(endian).into());
                result.e_phoff.set(NativeEndian, h.e_phoff(endian).into());
                result.e_shoff.set(NativeEndian, h.e_shoff(endian).into());
                result.e_flags.set(NativeEndian, h.e_flags(endian));
                result.e_ehsize.set(NativeEndian, h.e_ehsize(endian));
                result.e_phentsize.set(NativeEndian, h.e_phentsize(endian));
                result.e_phnum.set(NativeEndian, h.e_phnum(endian));
                result.e_shentsize.set(NativeEndian, h.e_shentsize(endian));
                result.e_shnum.set(NativeEndian, h.e_shnum(endian));
                result.e_shstrndx.set(NativeEndian, h.e_shstrndx(endian));
            }
            ParsedElf::Elf64(e) => {
                let h = e.elf_header();
                let endian = e.endian();
                result.e_ident = *h.e_ident();
                result.e_type.set(NativeEndian, h.e_type(endian));
                result.e_machine.set(NativeEndian, h.e_machine(endian));
                result.e_version.set(NativeEndian, h.e_version(endian));
                result.e_entry.set(NativeEndian, h.e_entry(endian));
                result.e_phoff.set(NativeEndian, h.e_phoff(endian));
                result.e_shoff.set(NativeEndian, h.e_shoff(endian));
                result.e_flags.set(NativeEndian, h.e_flags(endian));
                result.e_ehsize.set(NativeEndian, h.e_ehsize(endian));
                result.e_phentsize.set(NativeEndian, h.e_phentsize(endian));
                result.e_phnum.set(NativeEndian, h.e_phnum(endian));
                result.e_shentsize.set(NativeEndian, h.e_shentsize(endian));
                result.e_shnum.set(NativeEndian, h.e_shnum(endian));
                result.e_shstrndx.set(NativeEndian, h.e_shstrndx(endian));
            }
        }
        result
    }

    pub fn get_shdr(&self, index: usize) -> Option<GElf_Shdr> {
        let mut result: GElf_Shdr = unsafe { std::mem::zeroed() };
        match self {
            ParsedElf::Elf32(e) => {
                let table = e.elf_section_table();
                let endian = e.endian();
                table.section(SectionIndex(index)).ok().map(|s| {
                    result.sh_name.set(NativeEndian, s.sh_name(endian));
                    result.sh_type.set(NativeEndian, s.sh_type(endian));
                    result.sh_flags.set(NativeEndian, s.sh_flags(endian).into());
                    result.sh_addr.set(NativeEndian, s.sh_addr(endian).into());
                    result
                        .sh_offset
                        .set(NativeEndian, s.sh_offset(endian).into());
                    result.sh_size.set(NativeEndian, s.sh_size(endian).into());
                    result.sh_link.set(NativeEndian, s.sh_link(endian));
                    result.sh_info.set(NativeEndian, s.sh_info(endian));
                    result
                        .sh_addralign
                        .set(NativeEndian, s.sh_addralign(endian).into());
                    result
                        .sh_entsize
                        .set(NativeEndian, s.sh_entsize(endian).into());
                    result
                })
            }
            ParsedElf::Elf64(e) => {
                let table = e.elf_section_table();
                let endian = e.endian();
                table.section(SectionIndex(index)).ok().map(|s| {
                    result.sh_name.set(NativeEndian, s.sh_name(endian));
                    result.sh_type.set(NativeEndian, s.sh_type(endian));
                    result.sh_flags.set(NativeEndian, s.sh_flags(endian));
                    result.sh_addr.set(NativeEndian, s.sh_addr(endian));
                    result.sh_offset.set(NativeEndian, s.sh_offset(endian));
                    result.sh_size.set(NativeEndian, s.sh_size(endian));
                    result.sh_link.set(NativeEndian, s.sh_link(endian));
                    result.sh_info.set(NativeEndian, s.sh_info(endian));
                    result
                        .sh_addralign
                        .set(NativeEndian, s.sh_addralign(endian));
                    result.sh_entsize.set(NativeEndian, s.sh_entsize(endian));
                    result
                })
            }
        }
    }

    pub fn get_phdr(&self, index: usize) -> Option<GElf_Phdr> {
        let mut result: GElf_Phdr = unsafe { std::mem::zeroed() };
        match self {
            ParsedElf::Elf32(e) => {
                let phdrs = e.elf_program_headers();
                let endian = e.endian();
                phdrs.get(index).map(|p| {
                    result.p_type.set(NativeEndian, p.p_type(endian));
                    result.p_flags.set(NativeEndian, p.p_flags(endian));
                    result.p_offset.set(NativeEndian, p.p_offset(endian).into());
                    result.p_vaddr.set(NativeEndian, p.p_vaddr(endian).into());
                    result.p_paddr.set(NativeEndian, p.p_paddr(endian).into());
                    result.p_filesz.set(NativeEndian, p.p_filesz(endian).into());
                    result.p_memsz.set(NativeEndian, p.p_memsz(endian).into());
                    result.p_align.set(NativeEndian, p.p_align(endian).into());
                    result
                })
            }
            ParsedElf::Elf64(e) => {
                let phdrs = e.elf_program_headers();
                let endian = e.endian();
                phdrs.get(index).map(|p| {
                    result.p_type.set(NativeEndian, p.p_type(endian));
                    result.p_flags.set(NativeEndian, p.p_flags(endian));
                    result.p_offset.set(NativeEndian, p.p_offset(endian));
                    result.p_vaddr.set(NativeEndian, p.p_vaddr(endian));
                    result.p_paddr.set(NativeEndian, p.p_paddr(endian));
                    result.p_filesz.set(NativeEndian, p.p_filesz(endian));
                    result.p_memsz.set(NativeEndian, p.p_memsz(endian));
                    result.p_align.set(NativeEndian, p.p_align(endian));
                    result
                })
            }
        }
    }

    pub fn section_data(&self, index: usize) -> Option<&'a [u8]> {
        match self {
            ParsedElf::Elf32(e) => {
                let table = e.elf_section_table();
                let data = e.data();
                table
                    .section(SectionIndex(index))
                    .ok()
                    .and_then(|s| s.data(e.endian(), data).ok())
            }
            ParsedElf::Elf64(e) => {
                let table = e.elf_section_table();
                let data = e.data();
                table
                    .section(SectionIndex(index))
                    .ok()
                    .and_then(|s| s.data(e.endian(), data).ok())
            }
        }
    }
}

pub struct WriteState {
    pub obj: WriteObject<'static>,
    pub sections: Vec<object::write::SectionId>,
    pub section_data: Vec<Vec<u8>>,
    pub shstrtab_idx: Option<usize>,
    pub ehdr64: Option<Box<object::elf::FileHeader64<Endianness>>>,
}

pub struct Elf {
    pub fd: i32,
    pub cmd: ElfCmd,
    pub data: *const u8,
    pub data_len: usize,
    pub owned_data: Option<Vec<u8>>,
    pub mmap: Option<memmap2::MmapRaw>,
    pub parsed: Option<Box<ParsedElfOwned>>,
    pub section_handles: Vec<*mut Elf_Scn>,
    pub data_handles: Vec<*mut Elf_Data>,
    pub section_data_cache: Vec<Vec<u8>>,
    pub writer: Option<WriteState>,
}

pub struct ParsedElfOwned {
    inner: ParsedElf<'static>,
}

impl ParsedElfOwned {
    pub unsafe fn new(data: &[u8]) -> Result<Self, &'static str> {
        let static_data: &'static [u8] = unsafe { std::mem::transmute(data) };
        ParsedElf::parse(static_data).map(|inner| Self { inner })
    }

    pub fn get(&self) -> &ParsedElf<'static> {
        &self.inner
    }
}

impl Elf {
    pub fn ensure_parsed(&mut self) -> bool {
        if self.parsed.is_some() {
            return true;
        }

        let data = unsafe { std::slice::from_raw_parts(self.data, self.data_len) };
        match unsafe { ParsedElfOwned::new(data) } {
            Ok(parsed) => {
                self.parsed = Some(Box::new(parsed));
                true
            }
            Err(e) => {
                set_error(e);
                false
            }
        }
    }

    pub fn with_parsed<T>(&self, f: impl FnOnce(&ParsedElf) -> T) -> Option<T> {
        self.parsed.as_ref().map(|p| f(p.get()))
    }
}

impl Drop for Elf {
    fn drop(&mut self) {
        for handle in self.section_handles.drain(..) {
            unsafe {
                drop(Box::from_raw(handle));
            }
        }
        for handle in self.data_handles.drain(..) {
            unsafe {
                drop(Box::from_raw(handle));
            }
        }
    }
}

#[repr(C)]
pub struct Elf_Scn {
    pub elf: *mut Elf,
    pub index: usize,
    pub data_list_head: *mut Elf_Data,
    pub flags: u32,
}

impl Elf_Scn {
    pub fn new(elf: *mut Elf, index: usize) -> Self {
        Self {
            elf,
            index,
            data_list_head: std::ptr::null_mut(),
            flags: 0,
        }
    }
}
