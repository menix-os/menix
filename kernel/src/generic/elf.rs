use crate::arch;
use bytemuck::{Pod, Zeroable};

// ELF Header Identification
pub const ELF_MAG: [u8; 4] = [0x7F, b'E', b'L', b'F'];
pub const EI_CLASS: usize = 4; // File class
pub const EI_DATA: usize = 5; // Data encoding
pub const EI_VERSION: usize = 6; // File version
pub const EI_OSABI: usize = 7; // OS/ABI identification
pub const EI_ABIVERSION: usize = 8; // ABI version
pub const EI_PAD: usize = 9; // Start of padding bytes
pub const EI_NIDENT: usize = 16; // Size of e_ident[]

// ELF Identification Type
pub const ELFCLASS32: u8 = 1;
pub const ELFCLASS64: u8 = 2;
pub const ELFCLASSNUM: u8 = 3;
pub const ELFDATA2LSB: u8 = 1;
pub const ELFDATA2MSB: u8 = 2;
pub const ELFDATANUM: u8 = 3;

#[cfg(target_arch = "x86_64")]
pub const EM_CURRENT: u16 = 0x3E;
#[cfg(target_arch = "aarch64")]
pub const EM_CURRENT: u16 = 0xB7;
#[cfg(target_arch = "riscv64")]
pub const EM_CURRENT: u16 = 0xF3;
#[cfg(target_arch = "loongarch64")]
pub const EM_CURRENT: u16 = 0x102;

pub const EV_NONE: u8 = 0;
pub const EV_CURRENT: u8 = 1;
pub const EV_NUM: u8 = 2;

pub const ELFOSABI_SYSV: u8 = 0; // System V ABI
pub const ELFOSABI_HPUX: u8 = 1; // HP-UX operating system
pub const ELFOSABI_STANDALONE: u8 = 255; // Standalone (embedded) application

// ELF Header Type
pub const ET_NONE: u8 = 0;
pub const ET_REL: u8 = 1;
pub const ET_EXEC: u8 = 2;
pub const ET_DYN: u8 = 3;
pub const ET_CORE: u8 = 4;

// Program Header Types
pub const PT_NULL: u32 = 0x00000000;
pub const PT_LOAD: u32 = 0x00000001;
pub const PT_DYNAMIC: u32 = 0x00000002;
pub const PT_INTERP: u32 = 0x00000003;
pub const PT_NOTE: u32 = 0x00000004;
pub const PT_SHLIB: u32 = 0x00000005;
pub const PT_PHDR: u32 = 0x00000006;
pub const PT_TLS: u32 = 0x00000007;
pub const PT_MODULE: u32 = 0x60000000;

// Program Header Flags
pub const PF_EXECUTE: u32 = 0x01;
pub const PF_WRITE: u32 = 0x02;
pub const PF_READ: u32 = 0x04;

// Dynamic table
pub const DT_NULL: u32 = 0; // ignored Marks the end of the dynamic array
pub const DT_NEEDED: u32 = 1; // d_val The string table offset of the name of a needed library.
pub const DT_PLTRELSZ: u32 = 2; // d_val Total size of the relocation entries associated with the procedure linkage table.
pub const DT_PLTGOT: u32 = 3; // d_ptr Contains an address associated with the linkage table.
pub const DT_HASH: u32 = 4; // d_ptr Address of the symbol hash table, described below.
pub const DT_STRTAB: u32 = 5; // d_ptr Address of the dynamic string table.
pub const DT_SYMTAB: u32 = 6; // d_ptr Address of the dynamic symbol table.
pub const DT_RELA: u32 = 7; // d_ptr Address of a relocation table with Elf64_Rela entries.
pub const DT_RELASZ: u32 = 8; // d_val Total size, in bytes, of the DT_RELA relocation table.
pub const DT_RELAENT: u32 = 9; // d_val Size, in bytes, of each DT_RELA relocation entry.
pub const DT_STRSZ: u32 = 10; // d_val Total size, in bytes, of the string table.
pub const DT_SYMENT: u32 = 11; // d_val Size, in bytes, of each symbol table entry.
pub const DT_INIT: u32 = 12; // d_ptr Address of the initialization function.
pub const DT_FINI: u32 = 13; // d_ptr Address of the termination function.
pub const DT_SONAME: u32 = 14; // d_val The string table offset of the name of this shared object.
pub const DT_RPATH: u32 = 15; // d_val The string table offset of a shared library search path string.
pub const DT_SYMBOLIC: u32 = 16; // ignored
pub const DT_REL: u32 = 17; // d_ptr Address of a relocation table with Elf64_Rel entries.
pub const DT_RELSZ: u32 = 18; // d_val Total size, in bytes, of the DT_REL relocation table.
pub const DT_RELENT: u32 = 19; // d_val Size, in bytes, of each DT_REL relocation entry.
pub const DT_PLTREL: u32 = 20; // d_val Type of relocation entry used for the procedure linkage table.
pub const DT_DEBUG: u32 = 21; // d_ptr Reserved for debugger use.
pub const DT_TEXTREL: u32 = 22; // ignored The relocation table contains relocations for a non-writable segment.
pub const DT_JMPREL: u32 = 23; // d_ptr Address of the relocations associated with the procedure linkage table.
pub const DT_BIND_NOW: u32 = 24; // ignored The dynamic loader should process all relocations before transferring control.
pub const DT_INIT_ARRAY: u32 = 25; // d_ptr Pointer to an array of pointers to initialization functions.
pub const DT_FINI_ARRAY: u32 = 26; // d_ptr Pointer to an array of pointers to termination functions.
pub const DT_INIT_ARRAYSZ: u32 = 27; // d_val Size, in bytes, of the array of initialization functions.
pub const DT_FINI_ARRAYSZ: u32 = 28; // d_val Size, in bytes, of the array of termination functions.
pub const DT_LOOS: u32 = 0x60000000;
pub const DT_HIOS: u32 = 0x6FFFFFFF;
pub const DT_LOPROC: u32 = 0x70000000;
pub const DT_HIPROC: u32 = 0x7FFFFFFF;

// Symbol bindings
pub const STB_LOCAL: u32 = 0; // Not visible outside the object file
pub const STB_GLOBAL: u32 = 1; // Global symbol, visible to all object files
pub const STB_WEAK: u32 = 2; // Global scope, but with lower precedence than global symbols
pub const STB_LOOS: u32 = 10; // Environment-specific use
pub const STB_HIOS: u32 = 12; //
pub const STB_LOPROC: u32 = 13; // Processor-specific use
pub const STB_HIPROC: u32 = 15; //

// Symbol types
pub const STT_NOTYPE: u32 = 0; // No type specified (e.g., an absolute symbol)
pub const STT_OBJECT: u32 = 1; // Data object
pub const STT_FUNC: u32 = 2; // Function entry point
pub const STT_SECTION: u32 = 3; // Symbol is associated with a section
pub const STT_FILE: u32 = 4; // Source file associated with the object file
pub const STT_LOOS: u32 = 10; // Environment-specific use
pub const STT_HIOS: u32 = 12; //
pub const STT_LOPROC: u32 = 13; // Processor-specific use
pub const STT_HIPROC: u32 = 15; //

// Auxvals
pub const AT_NULL: u32 = 0;
pub const AT_IGNORE: u32 = 1;
pub const AT_EXECFD: u32 = 2;
pub const AT_PHDR: u32 = 3;
pub const AT_PHENT: u32 = 4;
pub const AT_PHNUM: u32 = 5;
pub const AT_PAGESZ: u32 = 6;
pub const AT_BASE: u32 = 7;
pub const AT_FLAGS: u32 = 8;
pub const AT_ENTRY: u32 = 9;
pub const AT_NOTELF: u32 = 10;
pub const AT_UID: u32 = 11;
pub const AT_EUID: u32 = 12;
pub const AT_GID: u32 = 13;
pub const AT_EGID: u32 = 14;
pub const AT_L4_AUX: u32 = 0xf0;
pub const AT_L4_ENV: u32 = 0xf1;

#[cfg(target_pointer_width = "64")]
pub type ElfAddr = u64;
#[cfg(target_pointer_width = "64")]
pub type ElfOff = u64;
#[cfg(target_pointer_width = "32")]
pub type ElfAddr = u32;
#[cfg(target_pointer_width = "32")]
pub type ElfOff = u32;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[cfg(target_pointer_width = "64")]
pub struct ElfPhdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: ElfOff,
    pub p_vaddr: ElfAddr,
    pub p_paddr: ElfAddr,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}
#[cfg(target_pointer_width = "64")]
assert_size!(ElfPhdr, 56);

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[cfg(target_pointer_width = "32")]
pub struct ElfPhdr {
    p_type: u32,
    p_offset: ElfOff,
    p_vaddr: ElfAddr,
    p_paddr: ElfAddr,
    p_filesz: u32,
    p_memsz: u32,
    p_flags: u32,
    p_align: u32,
}
#[cfg(target_pointer_width = "32")]
assert_size!(ElfPhdr, 32);

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[cfg(target_pointer_width = "64")]
pub struct ElfSym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: ElfAddr,
    pub st_size: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[cfg(target_pointer_width = "32")]
pub struct ElfSym {
    st_name: u32,
    st_value: ElfAddr,
    st_size: u32,
    st_info: u8,
    st_other: u8,
    st_shndx: u16,
}

/// ELF header
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct ElfHdr {
    /// ELF identification
    pub e_ident: [u8; EI_NIDENT],
    /// Object file type
    pub e_type: u16,
    /// Machine type
    pub e_machine: u16,
    /// Object file version
    pub e_version: u32,
    /// Entry point address
    pub e_entry: ElfAddr,
    /// Program header offset
    pub e_phoff: ElfOff,
    /// Section header offset
    pub e_shoff: ElfOff,
    /// Processor-specific flags
    pub e_flags: u32,
    /// ELF header size
    pub e_ehsize: u16,
    /// Size of program header entry
    pub e_phentsize: u16,
    /// Number of program header entries
    pub e_phnum: u16,
    /// Size of section header entry
    pub e_shentsize: u16,
    /// Number of section header entries
    pub e_shnum: u16,
    /// Section name string table index
    pub e_shstrndx: u16,
}
assert_size!(ElfHdr, 64);

pub struct ElfAuxv {
    atype: u32,
    avalue: u32,
}
