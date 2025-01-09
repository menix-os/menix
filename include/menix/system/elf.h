// Universal ELF32 and ELF64 definitions

#pragma once

#include <menix/common.h>
#include <menix/fs/handle.h>
#include <menix/memory/vm.h>

// ELF Header Identification
#define ELF_MAG				(const char[4]) {0x7F, 'E', 'L', 'F'}
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

#if MENIX_BITS == 64
#define ELF64_R_SYM(i)	   ((i) >> 32)
#define ELF64_R_TYPE(i)	   ((i) & 0xffffffffL)
#define ELF64_R_INFO(s, t) (((s) << 32) + ((t) & 0xffffffffL))
#endif
#define ELF32_R_SYM(i)	   ((i) >> 8)
#define ELF32_R_TYPE(i)	   ((u8)(i))
#define ELF32_R_INFO(s, t) (((s) << 8) + (u8)(t))

#if MENIX_BITS == 64
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

// Auxvals
#define AT_NULL	  0
#define AT_IGNORE 1
#define AT_EXECFD 2
#define AT_PHDR	  3
#define AT_PHENT  4
#define AT_PHNUM  5
#define AT_PAGESZ 6
#define AT_BASE	  7
#define AT_FLAGS  8
#define AT_ENTRY  9
#define AT_NOTELF 10
#define AT_UID	  11
#define AT_EUID	  12
#define AT_GID	  13
#define AT_EGID	  14
#define AT_L4_AUX 0xf0
#define AT_L4_ENV 0xf1

// Architecture specific ELF definitions
#if defined(__x86_64__)
#define EM_X86_64		   62
#define R_X86_64_NONE	   0	 //	None	None
#define R_X86_64_64		   1	 //	qword	S + A
#define R_X86_64_PC32	   2	 //	dword	S + A – P
#define R_X86_64_GOT32	   3	 //	dword	G + A
#define R_X86_64_PLT32	   4	 //	dword	L + A – P
#define R_X86_64_COPY	   5	 //	None	Value is copied directly from shared object
#define R_X86_64_GLOB_DAT  6	 //	qword	S
#define R_X86_64_JUMP_SLOT 7	 //	qword	S
#define R_X86_64_RELATIVE  8	 //	qword	B + A
#define R_X86_64_GOTPCREL  9	 //	dword	G + GOT + A – P
#define R_X86_64_32		   10	 //	dword	S + A
#define R_X86_64_32S	   11	 //	dword	S + A
#define R_X86_64_16		   12	 //	word	S + A
#define R_X86_64_PC16	   13	 //	word	S + A – P
#define R_X86_64_8		   14	 //	word8	S + A
#define R_X86_64_PC8	   15	 //	word8	S + A – P
#define R_X86_64_PC64	   24	 //	qword	S + A – P
#define R_X86_64_GOTOFF64  25	 //	qword	S + A – GOT
#define R_X86_64_GOTPC32   26	 //	dword	GOT + A – P
#define R_X86_64_SIZE32	   32	 //	dword	Z + A
#define R_X86_64_SIZE64	   33	 //	qword	Z + A
#define EI_ARCH_CLASS	   ELFCLASS64
#define EI_ARCH_DATA	   ELFDATA2LSB
#define EI_ARCH_MACHINE	   EM_X86_64

#elif defined(__aarch64__)
#elif defined(__riscv) && (__riscv_xlen == 64)
#define EM_RISCV				  243
#define R_RISCV_NONE			  0		//
#define R_RISCV_32				  1		// _word32_  S + A
#define R_RISCV_64				  2		// _word64_  S + A
#define R_RISCV_RELATIVE		  3		// _wordclass_ B + A
#define R_RISCV_COPY			  4		//
#define R_RISCV_JUMP_SLOT		  5		// _wordclass_ S
#define R_RISCV_TLS_DTPMOD32	  6		// _word32_ TLSMODULE
#define R_RISCV_TLS_DTPMOD64	  7		// _word64_ TLSMODULE
#define R_RISCV_TLS_DTPREL32	  8		// _word32_ S + A - TLS_DTV_OFFSET
#define R_RISCV_TLS_DTPREL64	  9		// _word64_ S + A - TLS_DTV_OFFSET
#define R_RISCV_TLS_TPREL32		  10	// _word32_ S + A + TLSOFFSET
#define R_RISCV_TLS_TPREL64		  11	// _word64_ S + A + TLSOFFSET
#define R_RISCV_TLSDESC			  12	// TLSDESC(S+A)
#define R_RISCV_BRANCH			  16	// _B-Type_ S + A - P
#define R_RISCV_JAL				  17	// _J-Type_ S + A - P
#define R_RISCV_CALL			  18	// _U+I-Type_ S + A - P
#define R_RISCV_CALL_PLT		  19	// _U+I-Type_ S + A - P
#define R_RISCV_GOT_HI20		  20	// _U-Type_ G + GOT + A - P
#define R_RISCV_TLS_GOT_HI20	  21	// _U-Type_
#define R_RISCV_TLS_GD_HI20		  22	// _U-Type_
#define R_RISCV_PCREL_HI20		  23	// _U-Type_ S + A - P
#define R_RISCV_PCREL_LO12_I	  24	// _I-type_ S - P
#define R_RISCV_PCREL_LO12_S	  25	// _S-Type_ S - P
#define R_RISCV_HI20			  26	// _U-Type_ S + A
#define R_RISCV_LO12_I			  27	// _I-Type_ S + A
#define R_RISCV_LO12_S			  28	// _S-Type_ S + A
#define R_RISCV_TPREL_HI20		  29	// _U-Type_
#define R_RISCV_TPREL_LO12_I	  30	// _I-Type_
#define R_RISCV_TPREL_LO12_S	  31	// _S-Type_
#define R_RISCV_TPREL_ADD		  32	//
#define R_RISCV_ADD8			  33	// _word8_ V + S + A
#define R_RISCV_ADD16			  34	// _word16_ V + S + A
#define R_RISCV_ADD32			  35	// _word32_ V + S + A
#define R_RISCV_ADD64			  36	// _word64_ V + S + A
#define R_RISCV_SUB8			  37	// _word8_ V - S - A
#define R_RISCV_SUB16			  38	// _word16_ V - S - A
#define R_RISCV_SUB32			  39	// _word32_ V - S - A
#define R_RISCV_SUB64			  40	// _word64_ V - S - A
#define R_RISCV_GOT32_PCREL		  41	// _word32_ G + GOT + A - P
#define R_RISCV_ALIGN			  43	//
#define R_RISCV_RVC_BRANCH		  44	// _CB-Type_ S + A - P
#define R_RISCV_RVC_JUMP		  45	// _CJ-Type_ S + A - P
#define R_RISCV_RELAX			  51	//
#define R_RISCV_SUB6			  52	// _word6_ V - S - A
#define R_RISCV_SET6			  53	// _word6_ S + A
#define R_RISCV_SET8			  54	// _word8_ S + A
#define R_RISCV_SET16			  55	// _word16_ S + A
#define R_RISCV_SET32			  56	// _word32_ S + A
#define R_RISCV_32_PCREL		  57	// _word32_ S + A - P
#define R_RISCV_IRELATIVE		  58	// _wordclass_ `ifunc_resolver(B + A)`
#define R_RISCV_PLT32			  59	// _word32_ S + A - P
#define R_RISCV_SET_ULEB128		  60	// _ULEB128_ S + A
#define R_RISCV_SUB_ULEB128		  61	// _ULEB128_ V - S - A
#define R_RISCV_TLSDESC_HI20	  62	// _U-Type_ S + A - P
#define R_RISCV_TLSDESC_LOAD_LO12 63	// _I-Type_ S - P
#define R_RISCV_TLSDESC_ADD_LO12  64	// _I-Type_ S - P
#define R_RISCV_TLSDESC_CALL	  65	//
#define EI_ARCH_CLASS			  ELFCLASS64
#define EI_ARCH_DATA			  ELFDATA2LSB
#define EI_ARCH_MACHINE			  EM_RISCV
#elif defined(__loongarch__) && (__loongarch_grlen == 64))
#endif

// ELF types that are related to the build host.
#if MENIX_BITS == 64
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
#define Elf_Auxv Elf64_Auxv
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
#define Elf_Auxv Elf32_Auxv
#endif

#if MENIX_BITS == 64
typedef u64 Elf64_Addr;
typedef u64 Elf64_Off;
#endif
typedef u32 Elf32_Addr;
typedef u32 Elf32_Off;

// The file header is located at the beginning of the file, and is used to locate the other parts of the file.
#if MENIX_BITS == 64
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
#if MENIX_BITS == 64
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
#if MENIX_BITS == 64
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
#if MENIX_BITS == 64
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
#if MENIX_BITS == 64
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
#if MENIX_BITS == 64
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
#if MENIX_BITS == 64
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
#if MENIX_BITS == 64
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

// Auxiliary Vector
#if MENIX_BITS == 64
typedef struct
{
	u32 atype;
	u32 avalue;
} Elf64_Auxv;
#else
typedef struct
{
	u32 atype;
	u32 avalue;
} Elf32_Auxv;
#endif

typedef struct
{
	Elf_Addr at_entry;
	Elf_Addr at_phdr;
	usize at_phent;
	usize at_phnum;
	char* ld_path;
} ElfInfo;

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
