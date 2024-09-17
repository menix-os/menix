// Universal ELF32 and ELF64 definitions

#pragma once

#include <menix/common.h>
#include <menix/fs/handle.h>
#include <menix/memory/vm.h>
#include <menix/util/log.h>

// ELF Header Identification
#define ELF_MAG \
	(const char[4]) \
	{ \
		0x7F, 'E', 'L', 'F' \
	}
#define EI_MAG0				0	  // 0x7F
#define EI_MAG1				1	  // 'E'
#define EI_MAG2				2	  // 'L'
#define EI_MAG3				3	  // 'F'
#define EI_CLASS			4	  // File class
#define EI_DATA				5	  // Data encoding
#define EI_VERSION			6	  // File version
#define EI_OSABI			7	  // OS/ABI identification
#define EI_ABIVERSION		8	  // ABI version
#define EI_PAD				9	  // Start of padding bytes
#define EI_NIDENT			16	  // Size of e_ident[]
// ELF Identification Type
#define ELFCLASS32			1
#define ELFCLASS64			2
#define ELFCLASSNUM			3
#define ELFDATA2LSB			1
#define ELFDATA2MSB			2
#define ELFDATANUM			3
#define EV_NONE				0
#define EV_CURRENT			1
#define EV_NUM				2
#define ELFOSABI_SYSV		0	   // System V ABI
#define ELFOSABI_HPUX		1	   // HP-UX operating system
#define ELFOSABI_STANDALONE 255	   // Standalone (embedded) application
// ELF Header Type
#define ET_NONE				0
#define ET_REL				1
#define ET_EXEC				2
#define ET_DYN				3
#define ET_CORE				4
#define ET_LOOS				0xFE00	  // Environment-specific use
#define ET_HIOS				0xFEFF	  //
#define ET_LOPROC			0xFF00	  // Processor-specific use
#define ET_HIPROC			0xFFFF	  //
// Program Header Type
#define PT_NULL				0x00000000
#define PT_LOAD				0x00000001
#define PT_DYNAMIC			0x00000002
#define PT_INTERP			0x00000003
#define PT_NOTE				0x00000004
#define PT_SHLIB			0x00000005
#define PT_PHDR				0x00000006
#define PT_TLS				0x00000007
#define PT_MODULE			0x61111111	  //! Custom type to mark the module segment/.mod section.
// Program Header Flags
#define PF_X				0x01
#define PF_W				0x02
#define PF_R				0x04
// Section Header
#define SHF_WRITE			0x1			  // Section contains writable data
#define SHF_ALLOC			0x2			  // Section is allocated in memory image of program
#define SHF_EXECINSTR		0x4			  // Section contains executable instructions
#define SHF_MASKOS			0x0F000000	  // Environment - specific use
#define SHF_MASKPROC		0xF0000000	  // Processor - specific use
// Section Header Linkage Type
#define SHT_NULL			0	  // Marks an unused section header
#define SHT_PROGBITS		1	  // Contains information defined by the program
#define SHT_SYMTAB			2	  // Contains a linker symbol table
#define SHT_STRTAB			3	  // Contains a string table
#define SHT_RELA			4	  // Contains “Rela” type relocation entries
#define SHT_HASH			5	  // Contains a symbol hash table
#define SHT_DYNAMIC			6	  // Contains dynamic linking tables
#define SHT_NOTE			7	  // Contains note information
#define SHT_NOBITS			8	  // Contains uninitialized space; does not occupy any space in the file
#define SHT_REL				9	  // Contains “Rel” type relocation entries
#define SHT_SHLIB			10	  // Reserved
#define SHT_DYNSYM			11	  // Contains a dynamic loader symbol table

#define ELF_ST_BIND(i)	  ((i) >> 4)
#define ELF_ST_TYPE(i)	  ((i) & 0xf)
#define ELF_ST_INFO(b, t) (((b) << 4) + ((t) & 0xf))

#if CONFIG_bits == 64
#define ELF64_R_SYM(i)	   ((i) >> 32)
#define ELF64_R_TYPE(i)	   ((i) & 0xffffffffL)
#define ELF64_R_INFO(s, t) (((s) << 32) + ((t) & 0xffffffffL))
#endif
#define ELF32_R_SYM(i)	   ((i) >> 8)
#define ELF32_R_TYPE(i)	   ((u8)(i))
#define ELF32_R_INFO(s, t) (((s) << 8) + (u8)(t))

#if CONFIG_bits == 64
#define ELF_R_SYM(i)	 ELF64_R_SYM(i)
#define ELF_R_TYPE(i)	 ELF64_R_TYPE(i)
#define ELF_R_INFO(s, t) ELF64_R_INFO(s, t)
#else
#define ELF_R_SYM(i)	 ELF32_R_SYM(i)
#define ELF_R_TYPE(i)	 ELF32_R_TYPE(i)
#define ELF_R_INFO(s, t) ELF32_R_INFO(s, t)
#endif

// Dynamic table
#define DT_NULL			0	  // ignored Marks the end of the dynamic array
#define DT_NEEDED		1	  // d_val The string table offset of the name of a needed library.
#define DT_PLTRELSZ		2	  // d_val Total size of the relocation entries associated with the procedure linkage table.
#define DT_PLTGOT		3	  // d_ptr Contains an address associated with the linkage table.
#define DT_HASH			4	  // d_ptr Address of the symbol hash table, described below.
#define DT_STRTAB		5	  // d_ptr Address of the dynamic string table.
#define DT_SYMTAB		6	  // d_ptr Address of the dynamic symbol table.
#define DT_RELA			7	  // d_ptr Address of a relocation table with Elf64_Rela entries.
#define DT_RELASZ		8	  // d_val Total size, in bytes, of the DT_RELA relocation table.
#define DT_RELAENT		9	  // d_val Size, in bytes, of each DT_RELA relocation entry.
#define DT_STRSZ		10	  // d_val Total size, in bytes, of the string table.
#define DT_SYMENT		11	  // d_val Size, in bytes, of each symbol table entry.
#define DT_INIT			12	  // d_ptr Address of the initialization function.
#define DT_FINI			13	  // d_ptr Address of the termination function.
#define DT_SONAME		14	  // d_val The string table offset of the name of this shared object.
#define DT_RPATH		15	  // d_val The string table offset of a shared library search path string.
#define DT_SYMBOLIC		16	  // ignored
#define DT_REL			17	  // d_ptr Address of a relocation table with Elf64_Rel entries.
#define DT_RELSZ		18	  // d_val Total size, in bytes, of the DT_REL relocation table.
#define DT_RELENT		19	  // d_val Size, in bytes, of each DT_REL relocation entry.
#define DT_PLTREL		20	  // d_val Type of relocation entry used for the procedure linkage table.
#define DT_DEBUG		21	  // d_ptr Reserved for debugger use.
#define DT_TEXTREL		22	  // ignored The relocation table contains relocations for a non-writable segment.
#define DT_JMPREL		23	  // d_ptr Address of the relocations associated with the procedure linkage table.
#define DT_BIND_NOW		24	  // ignored The dynamic loader should process all relocations before transferring control.
#define DT_INIT_ARRAY	25	  // d_ptr Pointer to an array of pointers to initialization functions.
#define DT_FINI_ARRAY	26	  // d_ptr Pointer to an array of pointers to termination functions.
#define DT_INIT_ARRAYSZ 27	  // d_val Size, in bytes, of the array of initialization functions.
#define DT_FINI_ARRAYSZ 28	  // d_val Size, in bytes, of the array of termination functions.
#define DT_LOOS			0x60000000
#define DT_HIOS			0x6FFFFFFF
#define DT_LOPROC		0x70000000
#define DT_HIPROC		0x7FFFFFFF

// Symbol bindings
#define STB_LOCAL  0	 // Not visible outside the object file
#define STB_GLOBAL 1	 // Global symbol, visible to all object files
#define STB_WEAK   2	 // Global scope, but with lower precedence than global symbols
#define STB_LOOS   10	 // Environment-specific use
#define STB_HIOS   12	 //
#define STB_LOPROC 13	 // Processor-specific use
#define STB_HIPROC 15	 //

// Symbol types
#define STT_NOTYPE	0	  // No type specified (e.g., an absolute symbol)
#define STT_OBJECT	1	  // Data object
#define STT_FUNC	2	  // Function entry point
#define STT_SECTION 3	  // Symbol is associated with a section
#define STT_FILE	4	  // Source file associated with the object file
#define STT_LOOS	10	  // Environment-specific use
#define STT_HIOS	12	  //
#define STT_LOPROC	13	  // Processor-specific use
#define STT_HIPROC	15	  //

// Architecture specific ELF definitions
#define MENIX_BITS_INCLUDE
#include <bits/elf.h>
#undef MENIX_BITS_INCLUDE

// ELF types that are related to the build host.
#if CONFIG_bits == 64
#define Elf_Hdr	 Elf64_Hdr
#define Elf_Phdr Elf64_Phdr
#define Elf_Dyn	 Elf64_Dyn
#define Elf_Ehdr Elf64_Ehdr
#define Elf_Shdr Elf64_Shdr
#define Elf_Addr Elf64_Addr
#define Elf_Off	 Elf64_Off
#define Elf_Sym	 Elf64_Sym
#define Elf_Rel	 Elf64_Rel
#define Elf_Rela Elf64_Rela
#define Elf_Nhdr Elf64_Nhdr
#else
#define Elf_Hdr	 Elf32_Hdr
#define Elf_Phdr Elf32_Phdr
#define Elf_Dyn	 Elf32_Dyn
#define Elf_Ehdr Elf32_Ehdr
#define Elf_Shdr Elf32_Shdr
#define Elf_Addr Elf32_Addr
#define Elf_Off	 Elf32_Off
#define Elf_Sym	 Elf32_Sym
#define Elf_Rel	 Elf32_Rel
#define Elf_Rela Elf32_Rela
#define Elf_Nhdr Elf32_Nhdr
#endif

#if CONFIG_bits == 64
typedef u64 Elf64_Addr;
typedef u64 Elf64_Off;
#endif
typedef u32 Elf32_Addr;
typedef u32 Elf32_Off;

// The file header is located at the beginning of the file, and is used to locate the other parts of the file.
#if CONFIG_bits == 64
typedef struct ATTR(packed)
{
	u8 e_ident[EI_NIDENT];	  // ELF identification
	u16 e_type;				  // Object file type
	u16 e_machine;			  // Machine type
	u32 e_version;			  // Object file version
	Elf64_Addr e_entry;		  // Entry point address
	Elf64_Off e_phoff;		  // Program header offset
	Elf64_Off e_shoff;		  // Section header offset
	u32 e_flags;			  // Processor-specific flags
	u16 e_ehsize;			  // ELF header size
	u16 e_phentsize;		  // Size of program header entry
	u16 e_phnum;			  // Number of program header entries
	u16 e_shentsize;		  // Size of section header entry
	u16 e_shnum;			  // Number of section header entries
	u16 e_shstrndx;			  // Section name string table index
} Elf64_Hdr;
#endif
typedef struct ATTR(packed)
{
	u8 e_ident[EI_NIDENT];	  // ELF identification
	u16 e_type;				  // Object file type
	u16 e_machine;			  // Machine type
	u32 e_version;			  // Object file version
	Elf32_Addr e_entry;		  // Entry point address
	Elf32_Off e_phoff;		  // Program header offset
	Elf32_Off e_shoff;		  // Section header offset
	u32 e_flags;			  // Processor-specific flags
	u16 e_ehsize;			  // ELF header size
	u16 e_phentsize;		  // Size of program header entry
	u16 e_phnum;			  // Number of program header entries
	u16 e_shentsize;		  // Size of section header entry
	u16 e_shnum;			  // Number of section header entries
	u16 e_shstrndx;			  // Section name string table index
} Elf32_Hdr;

// Program header. Field structure is different between bit sizes.
#if CONFIG_bits == 64
typedef struct ATTR(packed)
{
	u32 p_type;
	u32 p_flags;
	Elf64_Off p_offset;
	Elf64_Addr p_vaddr;
	Elf64_Addr p_paddr;
	u64 p_filesz;
	u64 p_memsz;
	u64 p_align;
} Elf64_Phdr;
#endif
typedef struct ATTR(packed)
{
	u32 p_type;
	Elf32_Off p_offset;
	Elf32_Addr p_vaddr;
	Elf32_Addr p_paddr;
	u32 p_filesz;
	u32 p_memsz;
	u32 p_flags;
	u32 p_align;
} Elf32_Phdr;

// Section header
#if CONFIG_bits == 64
typedef struct ATTR(packed)
{
	u32 sh_name;			// Section name
	u32 sh_type;			// Section type
	u64 sh_flags;			// Section attributes
	Elf64_Addr sh_addr;		// Virtual address in memory
	Elf64_Off sh_offset;	// Offset in file
	u64 sh_size;			// Size of section
	u32 sh_link;			// Link to other section
	u32 sh_info;			// Miscellaneous information
	u64 sh_addralign;		// Address alignment boundary
	u64 sh_entsize;			// Size of entries, if section has table
} Elf64_Shdr;
#endif
typedef struct ATTR(packed)
{
	u32 sh_name;			// Section name
	u32 sh_type;			// Section type
	u32 sh_flags;			// Section attributes
	Elf32_Addr sh_addr;		// Virtual address in memory
	Elf32_Off sh_offset;	// Offset in file
	u32 sh_size;			// Size of section
	u32 sh_link;			// Link to other section
	u32 sh_info;			// Miscellaneous information
	u32 sh_addralign;		// Address alignment boundary
	u32 sh_entsize;			// Size of entries, if section has table
} Elf32_Shdr;

// Symbol
#if CONFIG_bits == 64
typedef struct ATTR(packed)
{
	u32 st_name;
	u8 st_info;
	u8 st_other;
	u16 st_shndx;
	Elf64_Addr st_value;
	u64 st_size;
} Elf64_Sym;
#endif
typedef struct ATTR(packed)
{
	u32 st_name;
	Elf32_Addr st_value;
	u32 st_size;
	u8 st_info;
	u8 st_other;
	u16 st_shndx;
} Elf32_Sym;

// Dynamic entry
#if CONFIG_bits == 64
typedef struct ATTR(packed)
{
	i64 d_tag;
	union
	{
		u64 d_val;
		Elf64_Addr d_ptr;
	} d_un;
} Elf64_Dyn;
#endif
typedef struct ATTR(packed)
{
	i32 r_offset;
	union
	{
		u32 d_val;
		Elf32_Addr d_ptr;
	} d_un;
} Elf32_Dyn;

// Relocation
#if CONFIG_bits == 64
typedef struct ATTR(packed)
{
	Elf64_Addr r_offset;
	u64 r_info;
} Elf64_Rel;
#endif
typedef struct ATTR(packed)
{
	Elf32_Addr r_offset;
	u32 r_info;
} Elf32_Rel;

// Relocation + addend
#if CONFIG_bits == 64
typedef struct ATTR(packed)
{
	Elf64_Addr r_offset;
	u64 r_info;
	i64 r_addend;
} Elf64_Rela;
#endif
typedef struct ATTR(packed)
{
	Elf32_Addr r_offset;
	u32 r_info;
	i32 r_addend;
} Elf32_Rela;

// Note
#if CONFIG_bits == 64
typedef struct
{
	u64 n_namesz;
	u64 n_descsz;
	u64 n_type;
} Elf64_Nhdr;
#endif
typedef struct
{
	u32 n_namesz;
	u32 n_descsz;
	u32 n_type;
} Elf32_Nhdr;

typedef struct
{
	Elf_Addr entry_point;
	char* ld_path;
} ElfInfo;

#define elf_log(fmt, ...) kmesg("[ELF]\t" fmt, ##__VA_ARGS__)

// Gets a section from an ELF by name.
void* elf_get_section(void* elf, const char* name);

// Loads an ELF executable into memory. Returns true if successful.
// `page_map`: The page map of the process to map into.
// `handle`: A reference to a data stream of where to read the ELF from.
// `base`: If not 0: The image base where in virtual memory to load the executable.
// `info`: A reference to an ElfInfo structure to store important ELF information in.
bool elf_load(PageMap* page_map, Handle* handle, usize base, ElfInfo* info);

// Does a relocation on a symbol.
// `reloc`: The relocation to perform.
// `symtab_data`: Start of the symbol table.
// `strtab_data`: Start of the string table.
// `section_headers`: Start of the section header array.
// `base_virt`: The base address where the ELF was loaded.
i32 elf_do_reloc(Elf_Rela* reloc, Elf_Sym* symtab_data, const char* strtab_data, Elf_Shdr* section_headers,
				 void* base_virt);
