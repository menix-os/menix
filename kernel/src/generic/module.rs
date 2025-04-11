use super::{
    elf::{ElfHdr, ElfPhdr},
    memory::virt::VmFlags,
};
use crate::{
    arch::{PhysAddr, VirtAddr},
    boot::BootInfo,
    generic::{
        elf,
        memory::{
            PageAlloc,
            virt::{self, KERNEL_PAGE_TABLE},
        },
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
            print!("module: Loading \"{}\"\n", file.name);
            if let Err(x) = load(file.name, file.command_line, file.data) {
                print!("module: Failed to load module: {:?}\n", x);
            }
        }
    }
}

#[derive(Debug)]
pub enum ModuleLoadError {
    InvalidData,
    BrokenModuleInfo,
}

/// Loads a module from an ELF in memory.
pub fn load(name: &str, cmd: Option<&str>, data: &[u8]) -> Result<(), ModuleLoadError> {
    let elf_hdr: &ElfHdr = bytemuck::try_from_bytes(&data[0..size_of::<ElfHdr>()])
        .map_err(|_| ModuleLoadError::InvalidData)?;

    if elf_hdr.e_ident[0..4] != elf::ELF_MAG {
        return Err(ModuleLoadError::InvalidData);
    }

    #[cfg(target_pointer_width = "32")]
    if elf_hdr.e_ident[elf::EI_CLASS] != elf::ELFCLASS32 {
        return Err(ModuleLoadError::InvalidData);
    }

    #[cfg(target_pointer_width = "64")]
    if elf_hdr.e_ident[elf::EI_CLASS] != elf::ELFCLASS64 {
        return Err(ModuleLoadError::InvalidData);
    }

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

    if elf_hdr.e_ident[elf::EI_OSABI] != elf::ELFOSABI_SYSV {
        return Err(ModuleLoadError::InvalidData);
    }

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

    // Variables read from the dynamic segment.
    let mut dt_strtab = None;
    let mut dt_strsz = None;
    let mut dt_symtab = None;
    let mut dt_rela = None;
    let mut dt_relasz = None;
    let mut dt_relaent = None;
    let mut dt_pltrelsz = None;
    let mut dt_jmprel = None;
    let mut dt_init_array = None;
    let mut dt_init_arraysz = None;
    let mut dt_needed = Vec::new();

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

                let mut memory = Vec::new_in(PageAlloc);
                memory.resize(phdr.p_memsz as usize, 0u8);

                if load_base == 0 {
                    load_base = memory.as_ptr() as VirtAddr;
                }

                Vec::leak(memory);
            }
            elf::PT_DYNAMIC => {
                let dyntab: &[elf::ElfDyn] = bytemuck::try_cast_slice(
                    &data[phdr.p_offset as usize..(phdr.p_offset + phdr.p_filesz) as usize],
                )
                .map_err(|_| ModuleLoadError::InvalidData)?;

                for entry in dyntab {
                    match entry.d_tag as u32 {
                        elf::DT_STRTAB => dt_strtab = Some(entry.d_val),
                        elf::DT_SYMTAB => dt_symtab = Some(entry.d_val),
                        elf::DT_STRSZ => dt_strsz = Some(entry.d_val),
                        elf::DT_RELA => dt_rela = Some(entry.d_val),
                        elf::DT_RELASZ => dt_relasz = Some(entry.d_val),
                        elf::DT_RELAENT => dt_relaent = Some(entry.d_val),
                        elf::DT_PLTRELSZ => dt_pltrelsz = Some(entry.d_val),
                        elf::DT_JMPREL => dt_jmprel = Some(entry.d_val),
                        elf::DT_INIT_ARRAY => dt_init_array = Some(entry.d_val),
                        elf::DT_INIT_ARRAYSZ => dt_init_arraysz = Some(entry.d_val),
                        elf::DT_NEEDED => dt_needed.push(entry.d_val),
                        elf::DT_NULL => break,
                        _ => (),
                    }
                }
            }
            elf::PT_MODVERS => {
                info.version = str::from_utf8(
                    &data
                        [phdr.p_offset as usize..(phdr.p_offset as usize + phdr.p_filesz as usize)],
                )
                .map_err(|x| ModuleLoadError::BrokenModuleInfo)?
                .to_owned();
            }
            elf::PT_MODAUTH => {
                info.author = str::from_utf8(
                    &data
                        [phdr.p_offset as usize..(phdr.p_offset as usize + phdr.p_filesz as usize)],
                )
                .map_err(|x| ModuleLoadError::BrokenModuleInfo)?
                .to_owned();
            }
            elf::PT_MODDESC => {
                info.description = str::from_utf8(
                    &data
                        [phdr.p_offset as usize..(phdr.p_offset as usize + phdr.p_filesz as usize)],
                )
                .map_err(|x| ModuleLoadError::BrokenModuleInfo)?
                .to_owned();
            }
            // Unknown or unhandled type. Do nothing.
            _ => (),
        }
    }

    // Fix up addresses to offsets in the binary without having to map them in memory.
    let fix_addr = |opt: &mut Option<u64>| {
        if let Some(x) = opt {
            for phdr in phdrs {
                if phdr.p_vaddr <= *x && phdr.p_vaddr + phdr.p_filesz > *x {
                    *x -= phdr.p_vaddr;
                    *x += phdr.p_offset;
                }
            }
        }
    };

    fix_addr(&mut dt_strtab);
    fix_addr(&mut dt_symtab);
    fix_addr(&mut dt_rela);
    fix_addr(&mut dt_jmprel);
    fix_addr(&mut dt_init_array);

    let dependencies = dt_needed
        .as_slice()
        .iter()
        .map(|x| {
            CStr::from_bytes_until_nul(&data[(dt_strtab.unwrap() + *x) as usize..])
                .unwrap()
                .to_str()
                .unwrap()
        })
        // "menix.kso" is the kernel itself. We don't actually have to load that :)
        //.filter(|x| *x != "menix.kso")
        .collect::<Vec<_>>();

    print!("module: Loaded module \"{}\":\n", name);
    print!("module:   Base Address | {:#x}\n", load_base);
    print!("module:   Description  | {}\n", info.description);
    print!("module:   Version      | {}\n", info.version);
    print!("module:   Author(s)    | {}\n", info.author);
    print!("module:   Dependencies | {:?}\n", dependencies);

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
        unsafe extern "C" fn _start(args: *const u8) {
            $entry();
        }
    };
}
