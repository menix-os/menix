use super::virt::PageTable;
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
const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;
const ELFCLASSNUM: u8 = 3;
const ELFDATA2LSB: u8 = 1;
const ELFDATA2MSB: u8 = 2;
const ELFDATANUM: u8 = 3;

#[cfg(target_arch = "x86_64")]
const EM_CURRENT: u16 = 0x3E;
#[cfg(target_arch = "aarch64")]
const EM_CURRENT: u16 = 0xB7;
#[cfg(target_arch = "riscv64")]
const EM_CURRENT: u16 = 0xF3;
#[cfg(target_arch = "loongarch64")]
const EM_CURRENT: u16 = 0x102;

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
    /// ELF identification
    e_ident: [u8; EI_NIDENT],
    /// Object file type
    e_type: u16,
    /// Machine type
    e_machine: u16,
    /// Object file version
    e_version: u32,
    /// Entry point address
    e_entry: ElfAddr,
    /// Program header offset
    e_phoff: ElfOff,
    /// Section header offset
    e_shoff: ElfOff,
    /// Processor-specific flags
    e_flags: u32,
    /// ELF header size
    e_ehsize: u16,
    /// Size of program header entry
    e_phentsize: u16,
    /// Number of program header entries
    e_phnum: u16,
    /// Size of section header entry
    e_shentsize: u16,
    /// Number of section header entries
    e_shnum: u16,
    /// Section name string table index
    e_shstrndx: u16,
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
pub fn load_from_memory<const K: bool>(map: &mut PageTable<K>, data: &[u8]) -> Result<(), Error> {
    let elf_hdr: ElfHdr = unsafe { core::ptr::read(data.as_ptr() as *const _) };

    // Check ELF magic.
    if elf_hdr.e_ident[0..4] != ELF_MAG {
        return Err(Error::InvalidContent);
    }

    // We only support 64-bit.
    if elf_hdr.e_ident[EI_CLASS] != ELFCLASS64 {
        return Err(Error::InvalidContent);
    }

    // Check endianness.
    #[cfg(target_endian = "little")]
    if elf_hdr.e_ident[EI_DATA] != ELFDATA2LSB {
        return Err(Error::InvalidContent);
    }
    #[cfg(target_endian = "big")]
    if elf_hdr.e_ident[EI_DATA] != ELFDATA2MSB {
        return Err(Error::InvalidContent);
    }

    if elf_hdr.e_ident[EI_VERSION] != ElfVersion::Current as u8 {
        return Err(Error::InvalidContent);
    }

    // Check ABI, we don't care about ABIVERSION.
    if elf_hdr.e_ident[EI_OSABI] != ElfOsAbi::SysV as u8 {
        return Err(Error::InvalidContent);
    }

    // Check executable type. We don't support relocatable files.
    if elf_hdr.e_type != ElfEt::Exec as u16 {
        return Err(Error::InvalidContent);
    }

    // Check machine type. Only load executables for this machine.
    if elf_hdr.e_machine != EM_CURRENT {
        return Err(Error::InvalidContent);
    }

    // TODO: Evaluate PHDRs.

    return Ok(());
}
