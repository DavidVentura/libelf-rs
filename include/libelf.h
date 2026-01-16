/* libelf.h - ELF object file access library */
#ifndef _LIBELF_H
#define _LIBELF_H 1

#include <stddef.h>
#include <stdint.h>
#include "elf.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Elf Elf;
typedef struct Elf_Scn Elf_Scn;

typedef enum {
    ELF_K_NONE = 0,
    ELF_K_AR = 1,
    ELF_K_COFF = 2,
    ELF_K_ELF = 3,
    ELF_K_NUM = 4
} Elf_Kind;

typedef enum {
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
    ELF_C_NUM = 13
} Elf_Cmd;

typedef enum {
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
    ELF_T_NUM = 27
} Elf_Type;

typedef struct {
    void *d_buf;
    Elf_Type d_type;
    unsigned int d_version;
    size_t d_size;
    int64_t d_off;
    size_t d_align;
} Elf_Data;

#define ELF_F_DIRTY 0x1
#define ELF_F_LAYOUT 0x4
#define ELF_F_PERMISSIVE 0x8

#define EV_NONE 0
#define EV_CURRENT 1

unsigned int elf_version(unsigned int ver);

Elf *elf_begin(int fd, Elf_Cmd cmd, Elf *ref_elf);
Elf *elf_memory(char *image, size_t size);
int elf_end(Elf *elf);

Elf_Kind elf_kind(Elf *elf);

Elf_Scn *elf_nextscn(Elf *elf, Elf_Scn *scn);
Elf_Scn *elf_getscn(Elf *elf, size_t index);
size_t elf_ndxscn(Elf_Scn *scn);

int elf_getshdrstrndx(Elf *elf, size_t *dst);
int elf_getphdrnum(Elf *elf, size_t *dst);
int elf_getshdrnum(Elf *elf, size_t *dst);

Elf_Data *elf_getdata(Elf_Scn *scn, Elf_Data *data);
Elf_Data *elf_rawdata(Elf_Scn *scn, Elf_Data *data);

char *elf_strptr(Elf *elf, size_t section, size_t offset);

unsigned int elf_flagdata(Elf_Data *data, Elf_Cmd cmd, unsigned int flags);
unsigned int elf_flagshdr(Elf_Scn *scn, Elf_Cmd cmd, unsigned int flags);

Elf64_Ehdr *elf64_getehdr(Elf *elf);
Elf64_Shdr *elf64_getshdr(Elf_Scn *scn);

int elf_errno(void);
const char *elf_errmsg(int error);

typedef Elf64_Ehdr GElf_Ehdr;

#ifdef __cplusplus
}
#endif

#endif /* libelf.h */
