/* gelf.h - Generic ELF interface */
#ifndef _GELF_H
#define _GELF_H 1

#include "elf.h"
#include "libelf.h"

typedef Elf64_Shdr GElf_Shdr;
typedef Elf64_Sym GElf_Sym;
typedef Elf64_Rel GElf_Rel;
typedef Elf64_Rela GElf_Rela;
typedef Elf64_Phdr GElf_Phdr;
typedef Elf64_Nhdr GElf_Nhdr;
typedef Elf64_Versym GElf_Versym;
typedef Elf64_Verdef GElf_Verdef;
typedef Elf64_Verdaux GElf_Verdaux;
typedef Elf64_Relr GElf_Relr;

int gelf_getclass(Elf *elf);

GElf_Ehdr *gelf_getehdr(Elf *elf, GElf_Ehdr *dst);
GElf_Shdr *gelf_getshdr(Elf_Scn *scn, GElf_Shdr *dst);
GElf_Phdr *gelf_getphdr(Elf *elf, int index, GElf_Phdr *dst);

GElf_Sym *gelf_getsym(Elf_Data *data, int ndx, GElf_Sym *dst);
GElf_Versym *gelf_getversym(Elf_Data *data, int ndx, GElf_Versym *dst);
GElf_Verdef *gelf_getverdef(Elf_Data *data, int offset, GElf_Verdef *dst);
GElf_Verdaux *gelf_getverdaux(Elf_Data *data, int offset, GElf_Verdaux *dst);

size_t gelf_getnote(Elf_Data *data, size_t offset, GElf_Nhdr *nhdr,
                    size_t *name_offset, size_t *desc_offset);

unsigned char GELF_ST_BIND(unsigned char info);
unsigned char GELF_ST_TYPE(unsigned char info);
unsigned char GELF_ST_INFO(unsigned char bind, unsigned char type);

#endif /* gelf.h */
