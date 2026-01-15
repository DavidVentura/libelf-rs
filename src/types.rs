use std::ffi::c_void;

pub use object::NativeEndian;
pub use object::elf::{
    ELFCLASS32, ELFCLASS64, ELFCLASSNONE, ELFDATA2LSB, ELFDATA2MSB, ELFDATANONE, ET_DYN, ET_EXEC,
    ET_REL, EV_CURRENT, EV_NONE, PF_R, PF_W, PF_X, PT_DYNAMIC, PT_INTERP, PT_LOAD, PT_NOTE,
    PT_NULL, PT_PHDR, SHF_ALLOC, SHF_EXECINSTR, SHF_WRITE, SHN_ABS, SHN_COMMON, SHN_UNDEF,
    SHT_DYNAMIC, SHT_DYNSYM, SHT_GNU_VERDEF, SHT_GNU_VERNEED, SHT_GNU_VERSYM, SHT_HASH, SHT_NOBITS,
    SHT_NOTE, SHT_NULL, SHT_PROGBITS, SHT_REL, SHT_RELA, SHT_SHLIB, SHT_STRTAB, SHT_SYMTAB,
    STB_GLOBAL, STB_LOCAL, STB_WEAK, STT_FILE, STT_FUNC, STT_NOTYPE, STT_OBJECT, STT_SECTION,
    VER_DEF_CURRENT, VER_NEED_CURRENT,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfKind {
    ELF_K_NONE = 0,
    ELF_K_AR = 1,
    ELF_K_COFF = 2,
    ELF_K_ELF = 3,
    ELF_K_NUM = 4,
}

pub const ELF_K_NONE: ElfKind = ElfKind::ELF_K_NONE;
pub const ELF_K_AR: ElfKind = ElfKind::ELF_K_AR;
pub const ELF_K_ELF: ElfKind = ElfKind::ELF_K_ELF;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfCmd {
    ELF_C_NULL = 0,
    ELF_C_READ = 1,
    ELF_C_RDWR = 2,
    ELF_C_WRITE = 3,
    ELF_C_CLR = 4,
    ELF_C_SET = 5,
    ELF_C_FDDONE = 6,
    ELF_C_FDREAD = 7,
    ELF_C_READ_MMAP = 8,
    ELF_C_RDWR_MMAP = 9,
    ELF_C_WRITE_MMAP = 10,
    ELF_C_READ_MMAP_PRIVATE = 11,
    ELF_C_EMPTY = 12,
    ELF_C_NUM = 13,
}

pub const ELF_C_NULL: ElfCmd = ElfCmd::ELF_C_NULL;
pub const ELF_C_READ: ElfCmd = ElfCmd::ELF_C_READ;
pub const ELF_C_RDWR: ElfCmd = ElfCmd::ELF_C_RDWR;
pub const ELF_C_WRITE: ElfCmd = ElfCmd::ELF_C_WRITE;
pub const ELF_C_READ_MMAP: ElfCmd = ElfCmd::ELF_C_READ_MMAP;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfType {
    ELF_T_BYTE = 0,
    ELF_T_ADDR = 1,
    ELF_T_DYN = 2,
    ELF_T_EHDR = 3,
    ELF_T_HALF = 4,
    ELF_T_OFF = 5,
    ELF_T_PHDR = 6,
    ELF_T_RELA = 7,
    ELF_T_REL = 8,
    ELF_T_SHDR = 9,
    ELF_T_SWORD = 10,
    ELF_T_SYM = 11,
    ELF_T_WORD = 12,
    ELF_T_XWORD = 13,
    ELF_T_SXWORD = 14,
    ELF_T_VDEF = 15,
    ELF_T_VDAUX = 16,
    ELF_T_VNEED = 17,
    ELF_T_VNAUX = 18,
    ELF_T_NHDR = 19,
    ELF_T_SYMINFO = 20,
    ELF_T_MOVE = 21,
    ELF_T_LIB = 22,
    ELF_T_GNUHASH = 23,
    ELF_T_AUXV = 24,
    ELF_T_CHDR = 25,
    ELF_T_NHDR8 = 26,
    ELF_T_NUM = 27,
}

pub const ELF_T_BYTE: ElfType = ElfType::ELF_T_BYTE;
pub const ELF_T_SYM: ElfType = ElfType::ELF_T_SYM;

pub const ELF_F_DIRTY: u32 = 0x1;
pub const ELF_F_LAYOUT: u32 = 0x4;
pub const ELF_F_PERMISSIVE: u32 = 0x8;

#[repr(C)]
#[derive(Debug)]
pub struct Elf_Data {
    pub d_buf: *mut c_void,
    pub d_type: ElfType,
    pub d_version: u32,
    pub d_size: usize,
    pub d_off: i64,
    pub d_align: usize,
}

impl Default for Elf_Data {
    fn default() -> Self {
        Self {
            d_buf: std::ptr::null_mut(),
            d_type: ELF_T_BYTE,
            d_version: EV_CURRENT.into(),
            d_size: 0,
            d_off: 0,
            d_align: 1,
        }
    }
}

pub type GElf_Ehdr = object::elf::FileHeader64<NativeEndian>;
pub type GElf_Shdr = object::elf::SectionHeader64<NativeEndian>;
pub type GElf_Phdr = object::elf::ProgramHeader64<NativeEndian>;
pub type GElf_Sym = object::elf::Sym64<NativeEndian>;
pub type GElf_Rel = object::elf::Rel64<NativeEndian>;
pub type GElf_Rela = object::elf::Rela64<NativeEndian>;
pub type GElf_Nhdr = object::elf::NoteHeader64<NativeEndian>;
pub type GElf_Verdef = object::elf::Verdef<NativeEndian>;
pub type GElf_Verdaux = object::elf::Verdaux<NativeEndian>;
pub type GElf_Versym = object::elf::Versym<NativeEndian>;

#[inline]
pub fn gelf_st_bind(info: u8) -> u8 {
    info >> 4
}

#[inline]
pub fn gelf_st_type(info: u8) -> u8 {
    info & 0xf
}

#[inline]
pub fn gelf_st_info(bind: u8, typ: u8) -> u8 {
    (bind << 4) | (typ & 0xf)
}

#[unsafe(no_mangle)]
pub extern "C" fn GELF_ST_BIND(info: u8) -> u8 {
    gelf_st_bind(info)
}

#[unsafe(no_mangle)]
pub extern "C" fn GELF_ST_TYPE(info: u8) -> u8 {
    gelf_st_type(info)
}

#[unsafe(no_mangle)]
pub extern "C" fn GELF_ST_INFO(bind: u8, typ: u8) -> u8 {
    gelf_st_info(bind, typ)
}
