#ifndef _MENIX_SYS_ELF_H
#define _MENIX_SYS_ELF_H

#include <bits/sys/elf.h>
#include <stdint.h>

#define ELF_MAG       (const char[4]){0x7F, 'E', 'L', 'F'}
#define EI_MAG0       0
#define EI_MAG1       1
#define EI_MAG2       2
#define EI_MAG3       3
#define EI_CLASS      4
#define EI_DATA       5
#define EI_VERSION    6
#define EI_OSABI      7
#define EI_ABIVERSION 8
#define EI_PAD        9
#define EI_NIDENT     16

enum {
    EM_X86_64 = 62,
    EM_RISCV = 243,
};

enum {
    ELFCLASS32 = 1,
    ELFCLASS64 = 2,
};

enum {
    ELFDATA2LSB = 1,
    ELFDATA2MSB = 2,
};

enum {
    EV_NONE = 0,
    EV_CURRENT = 1,
    EV_NUM = 2,
};

enum {
    ELFOSABI_SYSV = 0,
    ELFOSABI_HPUX = 1,
    ELFOSABI_STANDALONE = 255,
};

enum {
    ET_NONE = 0,
    ET_REL = 1,
    ET_EXEC = 2,
    ET_DYN = 3,
    ET_CORE = 4,
};

enum {
    PT_NULL = 0x00000000,
    PT_LOAD = 0x00000001,
    PT_DYNAMIC = 0x00000002,
    PT_INTERP = 0x00000003,
    PT_NOTE = 0x00000004,
    PT_SHLIB = 0x00000005,
    PT_PHDR = 0x00000006,
    PT_TLS = 0x00000007,
};

enum {
    PF_X = 0x01,
    PF_W = 0x02,
    PF_R = 0x04,
};

enum {
    DT_NULL = 0,
    DT_NEEDED = 1,
    DT_PLTRELSZ = 2,
    DT_PLTGOT = 3,
    DT_HASH = 4,
    DT_STRTAB = 5,
    DT_SYMTAB = 6,
    DT_RELA = 7,
    DT_RELASZ = 8,
    DT_RELAENT = 9,
    DT_STRSZ = 10,
    DT_SYMENT = 11,
    DT_INIT = 12,
    DT_FINI = 13,
    DT_SONAME = 14,
    DT_RPATH = 15,
    DT_SYMBOLIC = 16,
    DT_REL = 17,
    DT_RELSZ = 18,
    DT_RELENT = 19,
    DT_PLTREL = 20,
    DT_DEBUG = 21,
    DT_TEXTREL = 22,
    DT_JMPREL = 23,
    DT_BIND_NOW = 24,
    DT_INIT_ARRAY = 25,
    DT_FINI_ARRAY = 26,
    DT_INIT_ARRAYSZ = 27,
    DT_FINI_ARRAYSZ = 28,
    DT_LOOS = 0x60000000,
    DT_HIOS = 0x6FFFFFFF,
    DT_LOPROC = 0x70000000,
    DT_HIPROC = 0x7FFFFFFF,
};

enum {
    AT_NULL = 0,
    AT_IGNORE = 1,
    AT_EXECFD = 2,
    AT_PHDR = 3,
    AT_PHENT = 4,
    AT_PHNUM = 5,
    AT_PAGESZ = 6,
    AT_BASE = 7,
    AT_FLAGS = 8,
    AT_ENTRY = 9,
    AT_NOTELF = 10,
    AT_UID = 11,
    AT_EUID = 12,
    AT_GID = 13,
    AT_EGID = 14,
};

#if __SIZEOF_POINTER__ == 8
#define Elf_Hdr  Elf64_Hdr
#define Elf_Phdr Elf64_Phdr
#define Elf_Dyn  Elf64_Dyn
#define Elf_Addr Elf64_Addr
#define Elf_Off  Elf64_Off
#define Elf_Nhdr Elf64_Nhdr
#define Elf_Auxv Elf64_Auxv
#else
#define Elf_Hdr  Elf32_Hdr
#define Elf_Phdr Elf32_Phdr
#define Elf_Dyn  Elf32_Dyn
#define Elf_Addr Elf32_Addr
#define Elf_Off  Elf32_Off
#define Elf_Nhdr Elf32_Nhdr
#define Elf_Auxv Elf32_Auxv
#endif

typedef uint64_t Elf64_Addr;
typedef uint64_t Elf64_Off;
typedef uint32_t Elf32_Addr;
typedef uint32_t Elf32_Off;

typedef struct {
    uint8_t e_ident[EI_NIDENT];
    uint16_t e_type;
    uint16_t e_machine;
    uint32_t e_version;
    Elf64_Addr e_entry;
    Elf64_Off e_phoff;
    Elf64_Off e_shoff;
    uint32_t e_flags;
    uint16_t e_ehsize;
    uint16_t e_phentsize;
    uint16_t e_phnum;
    uint16_t e_shentsize;
    uint16_t e_shnum;
    uint16_t e_shstrndx;
} Elf64_Ehdr;

typedef struct {
    uint8_t e_ident[EI_NIDENT];
    uint16_t e_type;
    uint16_t e_machine;
    uint32_t e_version;
    Elf32_Addr e_entry;
    Elf32_Off e_phoff;
    Elf32_Off e_shoff;
    uint32_t e_flags;
    uint16_t e_ehsize;
    uint16_t e_phentsize;
    uint16_t e_phnum;
    uint16_t e_shentsize;
    uint16_t e_shnum;
    uint16_t e_shstrndx;
} Elf32_Ehdr;

typedef struct {
    uint32_t p_type;
    uint32_t p_flags;
    Elf64_Off p_offset;
    Elf64_Addr p_vaddr;
    Elf64_Addr p_paddr;
    uint64_t p_filesz;
    uint64_t p_memsz;
    uint64_t p_align;
} Elf64_Phdr;

typedef struct {
    uint32_t p_type;
    Elf32_Off p_offset;
    Elf32_Addr p_vaddr;
    Elf32_Addr p_paddr;
    uint32_t p_filesz;
    uint32_t p_memsz;
    uint32_t p_flags;
    uint32_t p_align;
} Elf32_Phdr;

typedef struct {
    int64_t d_tag;
    union {
        uint64_t d_val;
        Elf64_Addr d_ptr;
    } d_un;
} Elf64_Dyn;

typedef struct {
    int32_t r_offset;
    union {
        uint32_t d_val;
        Elf32_Addr d_ptr;
    } d_un;
} Elf32_Dyn;

typedef struct {
    uint64_t n_namesz;
    uint64_t n_descsz;
    uint64_t n_type;
} Elf64_Nhdr;

typedef struct {
    uint32_t n_namesz;
    uint32_t n_descsz;
    uint32_t n_type;
} Elf32_Nhdr;

typedef struct {
    uint32_t atype;
    uint32_t avalue;
} Elf64_Auxv;

typedef struct {
    uint32_t atype;
    uint32_t avalue;
} Elf32_Auxv;

#endif
