use super::{
    elf::{ElfHdr, ElfPhdr},
    memory::virt::VmFlags,
};
use crate::{
    arch::{PhysAddr, VirtAddr},
    boot::BootInfo,
    generic::{
        elf,
        memory::virt::{self, KERNEL_PAGE_TABLE},
    },
};
use alloc::{borrow::ToOwned, collections::btree_map::BTreeMap, format, string::String, vec::Vec};
use core::{ffi::CStr, slice};
use spin::{Mutex, RwLock};

static SYMBOL_TABLE: RwLock<BTreeMap<String, (elf::ElfSym, Option<&ModuleInfo>)>> =
    RwLock::new(BTreeMap::new());
static MODULE_TABLE: RwLock<BTreeMap<String, ModuleInfo>> = RwLock::new(BTreeMap::new());

/// Stores metadata about a module.
#[derive(Debug)]
pub struct ModuleInfo {
    version: String,
    description: String,
    author: String,
    mappings: Vec<(PhysAddr, VirtAddr, usize, VmFlags)>,
}

/// Sets up the module system.
pub fn init(info: &mut BootInfo) {
    let dynsym_start = &raw const virt::LD_DYNSYM_START;
    let dynsym_end = &raw const virt::LD_DYNSYM_END;
    let dynstr_start = &raw const virt::LD_DYNSTR_START;
    let dynstr_end = &raw const virt::LD_DYNSTR_END;

    // Add all kernel symbols to a table so we can perform dynamic linking.
    {
        let symbols = unsafe {
            slice::from_raw_parts(
                dynsym_start as *const elf::ElfSym,
                (dynsym_end as VirtAddr - dynsym_start as VirtAddr) / size_of::<elf::ElfSym>(),
            )
        };
        let strings = unsafe {
            slice::from_raw_parts(
                dynstr_start,
                dynstr_end as VirtAddr - dynstr_start as VirtAddr,
            )
        };

        let mut symbol_table = SYMBOL_TABLE.write();
        let mut idx = 0;
        for sym in symbols {
            let name = unsafe { CStr::from_bytes_until_nul(&strings[sym.st_name as usize..]) };
            if let Ok(x) = name {
                if let Ok(s) = x.to_str() {
                    if !s.is_empty() {
                        assert!(symbol_table.insert(s.to_owned(), (*sym, None)).is_none());
                    }
                }
            }
        }
        print!(
            "module: Registered {} kernel symbols.\n",
            symbol_table.len()
        );
    }

    // Load all modules provided by the bootloader.
    if let Some(files) = info.files {
        for file in files {
            load(file.name, file.command_line, file.data);
        }
    }
}

pub enum ModuleLoadError {
    InvalidData,
    InvalidVersion,
}

/// Loads a module from an ELF in memory.
pub fn load(name: &str, cmd: Option<&str>, data: &[u8]) -> Result<(), ModuleLoadError> {
    let elf_hdr: &ElfHdr = bytemuck::try_from_bytes(&data[0..size_of::<ElfHdr>()])
        .map_err(|_| ModuleLoadError::InvalidData)?;

    // Check ELF magic.
    if elf_hdr.e_ident[0..4] != elf::ELF_MAG {
        return Err(ModuleLoadError::InvalidData);
    }

    // We only support 64-bit.
    if elf_hdr.e_ident[elf::EI_CLASS] != elf::ELFCLASS64 {
        return Err(ModuleLoadError::InvalidData);
    }

    // Check endianness.
    #[cfg(target_endian = "little")]
    if elf_hdr.e_ident[elf::EI_DATA] != elf::ELFDATA2LSB {
        return Err(ModuleLoadError::InvalidData);
    }
    #[cfg(target_endian = "big")]
    if elf_hdr.e_ident[EI_DATA] != ELFDATA2MSB {
        return Err(ModuleLoadError::InvalidData);
    }

    if elf_hdr.e_ident[elf::EI_VERSION] != elf::EV_CURRENT {
        return Err(ModuleLoadError::InvalidData);
    }

    // Check ABI, we don't care about ABIVERSION.
    if elf_hdr.e_ident[elf::EI_OSABI] != elf::ELFOSABI_SYSV {
        return Err(ModuleLoadError::InvalidData);
    }

    // Check machine type. Only load executables for this machine.
    if elf_hdr.e_machine != elf::EM_CURRENT {
        return Err(ModuleLoadError::InvalidData);
    }

    // Start by evaluating the program headers.
    let phdrs: &[ElfPhdr] = bytemuck::try_cast_slice(
        &data[elf_hdr.e_phoff as usize
            ..(elf_hdr.e_phoff as usize + elf_hdr.e_phnum as usize * size_of::<ElfPhdr>())],
    )
    .map_err(|_| ModuleLoadError::InvalidData)?;

    let mut load_base = 0; // TODO
    let mut info = ModuleInfo {
        version: String::new(),
        description: String::new(),
        author: String::new(),
        mappings: Vec::new(),
    };

    for phdr in phdrs {
        match phdr.p_type {
            // Load the segment into memory.
            elf::PT_LOAD => {
                // Convert the flags to our format.
                let mut flags = VmFlags::None;
                if phdr.p_flags & elf::PF_EXECUTE != 0 {
                    flags |= VmFlags::Exec;
                }
                if phdr.p_flags & elf::PF_READ != 0 {
                    flags |= VmFlags::Read;
                }
                if phdr.p_flags & elf::PF_WRITE != 0 {
                    flags |= VmFlags::Write;
                }

                // TODO
            }
            elf::PT_MODVERS => {
                info.version = str::from_utf8(
                    &data
                        [phdr.p_offset as usize..(phdr.p_offset as usize + phdr.p_filesz as usize)],
                )
                .map_err(|x| ModuleLoadError::InvalidVersion)?
                .to_owned();
            }
            elf::PT_MODAUTH => {
                info.author = str::from_utf8(
                    &data
                        [phdr.p_offset as usize..(phdr.p_offset as usize + phdr.p_filesz as usize)],
                )
                .map_err(|x| ModuleLoadError::InvalidVersion)?
                .to_owned();
            }
            elf::PT_MODDESC => {
                info.description = str::from_utf8(
                    &data
                        [phdr.p_offset as usize..(phdr.p_offset as usize + phdr.p_filesz as usize)],
                )
                .map_err(|x| ModuleLoadError::InvalidVersion)?
                .to_owned();
            }
            // Unknown or unhandled type. Do nothing.
            _ => (),
        }
    }

    print!(
        "module: Loaded {} at {:#x}: version = {}, args = {}\n",
        name,
        load_base,
        info.version,
        match cmd {
            Some(x) => x,
            None => "n/a",
        }
    );

    return Ok(());
}

#[macro_export]
macro_rules! module {
    ($desc: expr, $auth: expr, $entry: ident) => {
        const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

        #[unsafe(link_section = ".mod.vers")]
        #[used]
        static MODULE_VERS: [u8; MODULE_VERSION.len()] = {
            let mut buf = [0u8; MODULE_VERSION.len()];
            let src = MODULE_VERSION.as_bytes();
            let mut idx = 0;
            while idx < src.len() {
                buf[idx] = src[idx];
                idx += 1;
            }
            buf
        };

        #[unsafe(link_section = ".mod.desc")]
        #[used]
        static MODULE_DESC: [u8; $desc.len()] = {
            let mut buf = [0u8; $desc.len()];
            let src = $desc.as_bytes();
            let mut idx = 0;
            while idx < src.len() {
                buf[idx] = src[idx];
                idx += 1;
            }
            buf
        };

        #[unsafe(link_section = ".mod.auth")]
        #[used]
        static MODULE_AUTH: [u8; $auth.len()] = {
            let mut buf = [0u8; $auth.len()];
            let src = $auth.as_bytes();
            let mut idx = 0;
            while idx < src.len() {
                buf[idx] = src[idx];
                idx += 1;
            }
            buf
        };

        #[unsafe(no_mangle)]
        unsafe extern "C" fn _start() {
            $entry();
        }
    };
}
