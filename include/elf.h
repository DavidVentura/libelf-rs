/* elf.h - Standard ELF types */
#ifndef _ELF_H
#define _ELF_H 1

#include <stdint.h>

/* 64-bit ELF base types */
typedef uint64_t Elf64_Addr;
typedef uint16_t Elf64_Half;
typedef uint64_t Elf64_Off;
typedef int32_t  Elf64_Sword;
typedef int64_t  Elf64_Sxword;
typedef uint32_t Elf64_Word;
typedef uint64_t Elf64_Xword;

/* Relative relocation type */
typedef Elf64_Xword Elf64_Relr;

/* 32-bit ELF base types */
typedef uint32_t Elf32_Addr;
typedef uint16_t Elf32_Half;
typedef uint32_t Elf32_Off;
typedef int32_t  Elf32_Sword;
typedef uint32_t Elf32_Word;

/* 64-bit ELF header */
typedef struct {
    unsigned char e_ident[16];
    Elf64_Half    e_type;
    Elf64_Half    e_machine;
    Elf64_Word    e_version;
    Elf64_Addr    e_entry;
    Elf64_Off     e_phoff;
    Elf64_Off     e_shoff;
    Elf64_Word    e_flags;
    Elf64_Half    e_ehsize;
    Elf64_Half    e_phentsize;
    Elf64_Half    e_phnum;
    Elf64_Half    e_shentsize;
    Elf64_Half    e_shnum;
    Elf64_Half    e_shstrndx;
} Elf64_Ehdr;

/* 64-bit section header */
typedef struct {
    Elf64_Word    sh_name;
    Elf64_Word    sh_type;
    Elf64_Xword   sh_flags;
    Elf64_Addr    sh_addr;
    Elf64_Off     sh_offset;
    Elf64_Xword   sh_size;
    Elf64_Word    sh_link;
    Elf64_Word    sh_info;
    Elf64_Xword   sh_addralign;
    Elf64_Xword   sh_entsize;
} Elf64_Shdr;

/* 64-bit symbol table entry */
typedef struct {
    Elf64_Word    st_name;
    unsigned char st_info;
    unsigned char st_other;
    Elf64_Half    st_shndx;
    Elf64_Addr    st_value;
    Elf64_Xword   st_size;
} Elf64_Sym;

/* 64-bit relocation entries */
typedef struct {
    Elf64_Addr    r_offset;
    Elf64_Xword   r_info;
} Elf64_Rel;

typedef struct {
    Elf64_Addr    r_offset;
    Elf64_Xword   r_info;
    Elf64_Sxword  r_addend;
} Elf64_Rela;

/* 64-bit program header */
typedef struct {
    Elf64_Word    p_type;
    Elf64_Word    p_flags;
    Elf64_Off     p_offset;
    Elf64_Addr    p_vaddr;
    Elf64_Addr    p_paddr;
    Elf64_Xword   p_filesz;
    Elf64_Xword   p_memsz;
    Elf64_Xword   p_align;
} Elf64_Phdr;

/* 64-bit note header */
typedef struct {
    Elf64_Word n_namesz;
    Elf64_Word n_descsz;
    Elf64_Word n_type;
} Elf64_Nhdr;

/* Version symbol information */
typedef Elf64_Half Elf64_Versym;

/* Version definition */
typedef struct {
    Elf64_Half    vd_version;
    Elf64_Half    vd_flags;
    Elf64_Half    vd_ndx;
    Elf64_Half    vd_cnt;
    Elf64_Word    vd_hash;
    Elf64_Word    vd_aux;
    Elf64_Word    vd_next;
} Elf64_Verdef;

typedef struct {
    Elf64_Word    vda_name;
    Elf64_Word    vda_next;
} Elf64_Verdaux;

/* ELF header constants */
#define EI_NIDENT 16

/* e_ident[] indexes */
#define EI_MAG0     0
#define EI_MAG1     1
#define EI_MAG2     2
#define EI_MAG3     3
#define EI_CLASS    4
#define EI_DATA     5
#define EI_VERSION  6
#define EI_OSABI    7
#define EI_ABIVERSION 8
#define EI_PAD      9

/* ELF magic number */
#define ELFMAG0     0x7f
#define ELFMAG1     'E'
#define ELFMAG2     'L'
#define ELFMAG3     'F'

/* ELF class */
#define ELFCLASSNONE  0
#define ELFCLASS32    1
#define ELFCLASS64    2

/* Data encoding */
#define ELFDATANONE   0
#define ELFDATA2LSB   1
#define ELFDATA2MSB   2

/* e_type */
#define ET_NONE   0
#define ET_REL    1
#define ET_EXEC   2
#define ET_DYN    3
#define ET_CORE   4

/* e_machine */
#define EM_NONE   0
#define EM_X86_64 62
#define EM_AARCH64 183

/* Section header types */
#define SHT_NULL          0
#define SHT_PROGBITS      1
#define SHT_SYMTAB        2
#define SHT_STRTAB        3
#define SHT_RELA          4
#define SHT_HASH          5
#define SHT_DYNAMIC       6
#define SHT_NOTE          7
#define SHT_NOBITS        8
#define SHT_REL           9
#define SHT_SHLIB         10
#define SHT_DYNSYM        11
#define SHT_LOOS          0x60000000
#define SHT_HIOS          0x6fffffff
#define SHT_LOPROC        0x70000000
#define SHT_HIPROC        0x7fffffff

/* GNU extensions */
#define SHT_GNU_VERSYM    0x6fffffff
#define SHT_GNU_VERDEF    0x6ffffffd
#define SHT_GNU_VERNEED   0x6ffffffe
/* Lowercase aliases */
#define SHT_GNU_versym    SHT_GNU_VERSYM
#define SHT_GNU_verdef    SHT_GNU_VERDEF
#define SHT_GNU_verneed   SHT_GNU_VERNEED

/* Section header flags */
#define SHF_WRITE            (1 << 0)
#define SHF_ALLOC            (1 << 1)
#define SHF_EXECINSTR        (1 << 2)
#define SHF_MERGE            (1 << 4)
#define SHF_STRINGS          (1 << 5)
#define SHF_INFO_LINK        (1 << 6)
#define SHF_LINK_ORDER       (1 << 7)
#define SHF_OS_NONCONFORMING (1 << 8)
#define SHF_GROUP            (1 << 9)
#define SHF_TLS              (1 << 10)
#define SHF_COMPRESSED       (1 << 11)

/* Special section indexes */
#define SHN_UNDEF     0
#define SHN_ABS       0xfff1
#define SHN_COMMON    0xfff2
#define SHN_LORESERVE 0xff00

/* Symbol binding */
#define STB_LOCAL   0
#define STB_GLOBAL  1
#define STB_WEAK    2

/* Symbol types */
#define STT_NOTYPE  0
#define STT_OBJECT  1
#define STT_FUNC    2
#define STT_SECTION 3
#define STT_FILE    4
#define STT_COMMON  5
#define STT_TLS     6

/* Symbol visibility */
#define STV_DEFAULT   0
#define STV_INTERNAL  1
#define STV_HIDDEN    2
#define STV_PROTECTED 3

/* Relocation table entry macros */
#define ELF64_R_SYM(i)    ((i) >> 32)
#define ELF64_R_TYPE(i)   ((i) & 0xffffffff)
#define ELF64_R_INFO(s,t) (((s) << 32) + ((t) & 0xffffffff))

#define ELF32_R_SYM(i)    ((i) >> 8)
#define ELF32_R_TYPE(i)   ((unsigned char)(i))
#define ELF32_R_INFO(s,t) (((s) << 8) + (unsigned char)(t))

/* Symbol table entry macros */
#define ELF64_ST_BIND(i)   ((i) >> 4)
#define ELF64_ST_TYPE(i)   ((i) & 0xf)
#define ELF64_ST_INFO(b,t) (((b) << 4) + ((t) & 0xf))
#define ELF64_ST_VISIBILITY(o) ((o) & 0x03)

#define ELF32_ST_BIND(i)   ((i) >> 4)
#define ELF32_ST_TYPE(i)   ((i) & 0xf)
#define ELF32_ST_INFO(b,t) (((b) << 4) + ((t) & 0xf))
#define ELF32_ST_VISIBILITY(o) ((o) & 0x03)

/* Program header types */
#define PT_NULL    0
#define PT_LOAD    1
#define PT_DYNAMIC 2
#define PT_INTERP  3
#define PT_NOTE    4
#define PT_SHLIB   5
#define PT_PHDR    6
#define PT_TLS     7

/* Program header flags */
#define PF_X  (1 << 0)
#define PF_W  (1 << 1)
#define PF_R  (1 << 2)

/* Version constants */
#define VER_DEF_CURRENT   1
#define VER_NEED_CURRENT  1

#endif /* elf.h */
