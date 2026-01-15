#ifndef LIBELF_H
#define LIBELF_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdint.h>
#include <stddef.h>

#define ELF_F_DIRTY 1

#define ELF_F_LAYOUT 4

#define ELF_F_PERMISSIVE 8

typedef enum ElfCmd {
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
} ElfCmd;

typedef enum ElfKind {
  ELF_K_NONE = 0,
  ELF_K_AR = 1,
  ELF_K_COFF = 2,
  ELF_K_ELF = 3,
  ELF_K_NUM = 4,
} ElfKind;

typedef enum ElfType {
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
} ElfType;

/* Standard libelf type aliases */
typedef ElfCmd Elf_Cmd;
typedef ElfKind Elf_Kind;
typedef ElfType Elf_Type;

/* Opaque types */
typedef struct Elf Elf;
typedef struct Elf_Scn Elf_Scn;
typedef struct Elf_Data Elf_Data;
typedef struct Elf_Arhdr Elf_Arhdr;
typedef struct Elf_Arsym Elf_Arsym;

/* Elf_Data structure */
struct Elf_Data {
    void *d_buf;
    ElfType d_type;
    size_t d_size;
    uint64_t d_off;
    size_t d_align;
    unsigned int d_version;
};

/* Archive member header */
struct Elf_Arhdr {
    char *ar_name;
    uint64_t ar_date;
    long ar_uid;
    long ar_gid;
    unsigned long ar_mode;
    int64_t ar_size;
    char *ar_rawname;
};

/* Archive symbol table */
struct Elf_Arsym {
    char *as_name;
    size_t as_off;
    unsigned long as_hash;
};

/* Library initialization */
unsigned int elf_version(unsigned int version);
#define EV_CURRENT 1
#define EV_NONE 0

/* Elf descriptor functions */
Elf *elf_begin(int fd, Elf_Cmd cmd, Elf *ref);
Elf *elf_memory(char *image, size_t size);
int elf_end(Elf *elf);
ElfKind elf_kind(Elf *elf);
int elf_update(Elf *elf, Elf_Cmd cmd);
int64_t elf_getphdrnum(Elf *elf, size_t *dst);
int64_t elf_getshdrnum(Elf *elf, size_t *dst);
int64_t elf_getshdrstrndx(Elf *elf, size_t *dst);

/* Section functions */
Elf_Scn *elf_getscn(Elf *elf, size_t index);
Elf_Scn *elf_nextscn(Elf *elf, Elf_Scn *scn);
size_t elf_ndxscn(Elf_Scn *scn);
Elf_Scn *elf_newscn(Elf *elf);

/* Data functions */
Elf_Data *elf_getdata(Elf_Scn *scn, Elf_Data *data);
Elf_Data *elf_newdata(Elf_Scn *scn);
Elf_Data *elf_rawdata(Elf_Scn *scn, Elf_Data *data);

/* Header functions */
char *elf_strptr(Elf *elf, size_t section, size_t offset);
void *elf_getehdr(Elf *elf);
void *elf_getshdr(Elf_Scn *scn);
void *elf_getphdr(Elf *elf);
void *elf_newehdr(Elf *elf);
void *elf_newphdr(Elf *elf, size_t count);

/* 32-bit specific functions */
void *elf32_getehdr(Elf *elf);
void *elf32_getphdr(Elf *elf);
void *elf32_getshdr(Elf_Scn *scn);
void *elf32_newehdr(Elf *elf);
void *elf32_newphdr(Elf *elf, size_t count);

/* 64-bit specific functions */
void *elf64_getehdr(Elf *elf);
void *elf64_getphdr(Elf *elf);
void *elf64_getshdr(Elf_Scn *scn);
void *elf64_newehdr(Elf *elf);
void *elf64_newphdr(Elf *elf, size_t count);

/* Archive functions */
Elf_Arhdr *elf_getarhdr(Elf *elf);
uint64_t elf_getaroff(Elf *elf);
Elf_Arsym *elf_getarsym(Elf *elf, size_t *ptr);
Elf *elf_next(Elf *elf);
Elf_Cmd elf_next_cmd(Elf *elf);
int elf_rand(Elf *elf, size_t offset);

/* Error handling */
unsigned int elf_errno(void);
const char *elf_errmsg(int error);

/* Flags */
unsigned int elf_flagdata(Elf_Data *data, Elf_Cmd cmd, unsigned int flags);
unsigned int elf_flagehdr(Elf *elf, Elf_Cmd cmd, unsigned int flags);
unsigned int elf_flagelf(Elf *elf, Elf_Cmd cmd, unsigned int flags);
unsigned int elf_flagphdr(Elf *elf, Elf_Cmd cmd, unsigned int flags);
unsigned int elf_flagscn(Elf_Scn *scn, Elf_Cmd cmd, unsigned int flags);
unsigned int elf_flagshdr(Elf_Scn *scn, Elf_Cmd cmd, unsigned int flags);

/* Raw data functions */
char *elf_rawfile(Elf *elf, size_t *size);

/* Hash functions */
unsigned long elf_hash(const unsigned char *name);
long elf32_checksum(Elf *elf);
long elf64_checksum(Elf *elf);

/* Fill byte */
void elf_fill(int fill);

#endif  /* LIBELF_H */
