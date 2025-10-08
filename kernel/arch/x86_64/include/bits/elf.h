#pragma once

#define ELF_ARCH_CLASS   ELFCLASS64
#define ELF_ARCH_DATA    ELFDATA2LSB
#define ELF_ARCH_MACHINE EM_X86_64

#define elf_hdr  elf64_hdr
#define elf_phdr elf64_phdr
#define elf_dyn  elf64_dyn
#define elf_addr elf64_addr
#define elf_off  elf64_off
#define elf_nhdr elf64_nhdr
#define elf_auxv elf64_auxv
