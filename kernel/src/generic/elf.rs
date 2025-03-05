use super::virt::GenericPageMap;
use bitflags::bitflags;
use portal::error::Error;

// ELF Header Identification
const ELF_MAG: [u8; 4] = [0x7F, 'E' as u8, 'L' as u8, 'F' as u8];
const EI_CLASS: usize = 4; // File class
const EI_DATA: usize = 5; // Data encoding
const EI_VERSION: usize = 6; // File version
const EI_OSABI: usize = 7; // OS/ABI identification
const EI_ABIVERSION: usize = 8; // ABI version
const EI_PAD: usize = 9; // Start of padding bytes
const EI_NIDENT: usize = 16; // Size of e_ident[]

// ELF Identification Type
const ELFCLASS32: usize = 1;
const ELFCLASS64: usize = 2;
const ELFCLASSNUM: usize = 3;
const ELFDATA2LSB: usize = 1;
const ELFDATA2MSB: usize = 2;
const ELFDATANUM: usize = 3;

#[repr(u8)]
pub enum ElfVersion {
    None = 0,
    Current = 1,
    Num = 2,
}

#[repr(u8)]
pub enum ElfOsAbi {
    /// System V ABI
    SysV = 0,
    /// HP-UX operating system
    HPUX = 1,
    /// Standalone (embedded) application
    Standalone = 255,
}

// ELF Header Type
#[repr(u8)]
pub enum ElfEt {
    None = 0,
    Rel = 1,
    Exec = 2,
    Dyn = 3,
    Core = 4,
}

/// Program Header Type
#[repr(u32)]
pub enum ElfPt {
    /// Do nothing with this
    Null = 0x00000000,
    /// Load and map into memory
    Load = 0x00000001,
    /// Dynamic segment
    Dynamic = 0x00000002,
    /// Interpreter path
    Interp = 0x00000003,
    /// Note
    Note = 0x00000004,
    /// Shared library
    Shlib = 0x00000005,
    /// Program headers
    Phdr = 0x00000006,
    /// Thread-local storage
    Tls = 0x00000007,
}

// Program Header Flags
bitflags! {
    pub struct ElfPf: u8 {
        const Execute = 0x01;
        const Write = 0x02;
        const Read = 0x04;
    }
}

pub type ElfAddr = usize;
pub type ElfOff = usize;

/// ELF header
#[repr(C, packed)]
struct ElfHdr {
    e_ident: [u8; EI_NIDENT], // ELF identification
    e_type: u16,              // Object file type
    e_machine: u16,           // Machine type
    e_version: u32,           // Object file version
    e_entry: ElfAddr,         // Entry point address
    e_phoff: ElfOff,          // Program header offset
    e_shoff: ElfOff,          // Section header offset
    e_flags: u32,             // Processor-specific flags
    e_ehsize: u16,            // ELF header size
    e_phentsize: u16,         // Size of program header entry
    e_phnum: u16,             // Number of program header entries
    e_shentsize: u16,         // Size of section header entry
    e_shnum: u16,             // Number of section header entries
    e_shstrndx: u16,          // Section name string table index
}
assert_size!(ElfHdr, 64);

#[repr(C, packed)]
pub struct ElfPhdr {
    p_type: u32,
    p_flags: u32,
    p_offset: ElfOff,
    p_vaddr: ElfAddr,
    p_paddr: ElfAddr,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

/// Loads a raw ELF executable from memory into a given page map.
pub fn load_from_memory(map: &mut impl GenericPageMap, data: &[u8]) -> Result<(), Error> {
    // Check ELF for validity.
    if data[0..=3] != ELF_MAG {
        return Err(Error::BadArgument);
    }

    return Ok(());
}
