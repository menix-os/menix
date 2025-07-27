use super::{
    memory::{
        PhysAddr, VirtAddr,
        pmm::{AllocFlags, KernelAlloc, PageAllocator},
        virt::VmFlags,
        virt::VmLevel,
    },
    util::spin_mutex::SpinMutex,
    util::{align_down, align_up},
    vfs::exec::elf::{self, ElfHashTable, ElfHdr, ElfPhdr, ElfRela, ElfSym},
};
use crate::{
    arch,
    generic::memory::virt::{self, PageTable},
};
use alloc::{borrow::ToOwned, collections::btree_map::BTreeMap, string::String, vec::Vec};
use core::{
    ffi::CStr,
    slice,
    sync::atomic::{AtomicUsize, Ordering},
};

// TODO: This can use RwLocks.
pub(crate) static SYMBOL_TABLE: SpinMutex<BTreeMap<String, (elf::ElfSym, Option<&ModuleInfo>)>> =
    SpinMutex::new(BTreeMap::new());

pub(crate) static MODULE_TABLE: SpinMutex<BTreeMap<String, ModuleInfo>> =
    SpinMutex::new(BTreeMap::new());

unsafe extern "C" {
    unsafe static LD_DYNSYM_START: u8;
    unsafe static LD_DYNSYM_END: u8;
    unsafe static LD_DYNSTR_START: u8;
    unsafe static LD_DYNSTR_END: u8;
}

type ModuleEntryFn = extern "C" fn();

/// Stores metadata about a module.
#[derive(Debug)]
pub struct ModuleInfo {
    pub version: String,
    pub description: String,
    pub author: String,
    pub entry: Option<ModuleEntryFn>,
    pub mappings: Vec<(PhysAddr, VirtAddr, usize, VmFlags)>,
}

/// Sets up the module system.
#[initgraph::task(
    name = "generic.module",
    depends = [super::memory::MEMORY_STAGE],
)]
fn MODULE_STAGE() {
    let dynsym_start = &raw const LD_DYNSYM_START;
    let dynsym_end = &raw const LD_DYNSYM_END;
    let dynstr_start = &raw const LD_DYNSTR_START;
    let dynstr_end = &raw const LD_DYNSTR_END;

    // Add all kernel symbols to a table so we can perform dynamic linking.
    {
        let symbols = unsafe {
            slice::from_raw_parts_mut(
                dynsym_start as *mut elf::ElfSym,
                (dynsym_end as usize - dynsym_start as usize) / size_of::<elf::ElfSym>(),
            )
        };
        let strings = unsafe {
            slice::from_raw_parts(dynstr_start, dynstr_end as usize - dynstr_start as usize)
        };

        let mut symbol_table = SYMBOL_TABLE.lock();
        for sym in symbols {
            // Fix the addresses in the symbols because relocating doesn't relocate the symbol address.
            sym.st_value += &raw const virt::LD_KERNEL_START as u64;

            let name = CStr::from_bytes_until_nul(&strings[sym.st_name as usize..]);
            if let Ok(x) = name
                && let Ok(s) = x.to_str()
                && !s.is_empty()
            {
                let result = symbol_table.insert(s.to_owned(), (*sym, None));
                assert!(result.is_none(), "Duplicate symbol names!");
            }
        }
        log!("Registered {} kernel symbols", symbol_table.len());
    }
}

#[derive(Debug)]
pub enum ModuleLoadError {
    InvalidData,
    BrokenModuleInfo,
    AllocFailed,
    UnsupportedRelocation,
    SymbolNotFound,
}

pub static MODULE_ADDR: AtomicUsize = AtomicUsize::new(0);

/// Loads a module from an ELF in memory.
pub fn load(name: &str, data: &[u8]) -> Result<(), ModuleLoadError> {
    let elf_hdr: &ElfHdr = bytemuck::try_from_bytes(&data[0..size_of::<ElfHdr>()])
        .map_err(|_| ModuleLoadError::InvalidData)?;

    if elf_hdr.e_ident[0..4] != elf::ELF_MAG
        || elf_hdr.e_ident[elf::EI_VERSION] != elf::EV_CURRENT
        || elf_hdr.e_ident[elf::EI_OSABI] != elf::ELFOSABI_SYSV
        || elf_hdr.e_machine != elf::EM_CURRENT
    {
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

    // Start by evaluating the program headers.
    let phdrs: &[ElfPhdr] = bytemuck::try_cast_slice(
        &data[elf_hdr.e_phoff as usize
            ..(elf_hdr.e_phoff as usize + elf_hdr.e_phnum as usize * size_of::<ElfPhdr>())],
    )
    .map_err(|_| ModuleLoadError::InvalidData)?;

    let mut load_base = 0;
    let mut info = ModuleInfo {
        version: String::new(),
        description: String::new(),
        author: String::new(),
        entry: None,
        mappings: Vec::new(),
    };

    // Variables read from the dynamic segment.
    let mut dt_strtab = None;
    let mut dt_strsz = None;
    let mut dt_symtab = None;
    let mut dt_rela = None;
    let mut dt_relasz = None;
    let mut dt_pltrelsz = None;
    let mut dt_jmprel = None;
    let mut dt_init_array = None;
    let mut dt_hash = None;
    let mut dt_needed = Vec::new();

    for phdr in phdrs {
        match phdr.p_type {
            // Load the segment into memory.
            elf::PT_LOAD => {
                // Record where the first PHDR was loaded at.
                if load_base == 0 {
                    load_base = MODULE_ADDR.load(Ordering::Acquire);
                }

                let mut memsz = phdr.p_memsz as usize;

                // Fix potentially unaligned addresses.
                let aligned_virt = align_down(
                    phdr.p_vaddr as usize,
                    arch::virt::get_page_size(VmLevel::L1),
                );
                if aligned_virt < phdr.p_vaddr as usize {
                    memsz += arch::virt::get_page_size(VmLevel::L1)
                        - (phdr.p_memsz as usize % arch::virt::get_page_size(VmLevel::L1));
                }

                // Allocate physical memory.
                let phys = KernelAlloc::alloc_bytes(memsz, AllocFlags::Zeroed)
                    .map_err(|_| ModuleLoadError::AllocFailed)?;

                let page_table = PageTable::get_kernel();

                // Map memory with RW permissions.
                for page in (0..memsz).step_by(arch::virt::get_page_size(VmLevel::L1)) {
                    page_table
                        .map_single::<KernelAlloc>(
                            (load_base + aligned_virt + page).into(),
                            phys + page,
                            VmFlags::Read | VmFlags::Write,
                            VmLevel::L1,
                        )
                        .map_err(|_| ModuleLoadError::AllocFailed)?;

                    MODULE_ADDR.fetch_add(arch::virt::get_page_size(VmLevel::L1), Ordering::AcqRel);
                }

                let virt = load_base + phdr.p_vaddr as usize;

                // Copy data to allocated memory.
                let buf =
                    unsafe { slice::from_raw_parts_mut(virt as *mut u8, phdr.p_memsz as usize) };
                buf.copy_from_slice(&data[phdr.p_offset as usize..][..phdr.p_filesz as usize]);
                buf[phdr.p_filesz as usize..].fill(0);

                // Convert the flags to our format.
                let mut flags = VmFlags::empty();
                if phdr.p_flags & elf::PF_EXECUTE != 0 {
                    flags |= VmFlags::Exec;
                }
                if phdr.p_flags & elf::PF_READ != 0 {
                    flags |= VmFlags::Read;
                }
                if phdr.p_flags & elf::PF_WRITE != 0 {
                    flags |= VmFlags::Write;
                }

                // Record this mapping.
                info.mappings
                    .push((phys, (load_base + aligned_virt).into(), memsz, flags));
            }
            elf::PT_DYNAMIC => {
                let dyntab: &[elf::ElfDyn] = bytemuck::try_cast_slice(
                    &data[phdr.p_offset as usize..][..phdr.p_filesz as usize],
                )
                .map_err(|_| ModuleLoadError::InvalidData)?;

                for entry in dyntab {
                    match entry.d_tag as u32 {
                        elf::DT_STRTAB => dt_strtab = Some(entry.d_val),
                        elf::DT_SYMTAB => dt_symtab = Some(entry.d_val),
                        elf::DT_STRSZ => dt_strsz = Some(entry.d_val),
                        elf::DT_RELA => dt_rela = Some(entry.d_val),
                        elf::DT_RELASZ => dt_relasz = Some(entry.d_val),
                        elf::DT_PLTRELSZ => dt_pltrelsz = Some(entry.d_val),
                        elf::DT_JMPREL => dt_jmprel = Some(entry.d_val),
                        elf::DT_INIT_ARRAY => dt_init_array = Some(entry.d_val),
                        elf::DT_HASH => dt_hash = Some(entry.d_val),
                        elf::DT_NEEDED => dt_needed.push(entry.d_val),
                        elf::DT_NULL => break,
                        _ => (),
                    }
                }
            }
            elf::PT_MODVERSION => {
                info.version =
                    str::from_utf8(&data[phdr.p_offset as usize..][..phdr.p_filesz as usize])
                        .map_err(|_| ModuleLoadError::BrokenModuleInfo)?
                        .to_owned();
            }
            elf::PT_MODAUTHOR => {
                info.author =
                    str::from_utf8(&data[phdr.p_offset as usize..][..phdr.p_filesz as usize])
                        .map_err(|_| ModuleLoadError::BrokenModuleInfo)?
                        .to_owned();
            }
            elf::PT_MODDESC => {
                info.description =
                    str::from_utf8(&data[phdr.p_offset as usize..][..phdr.p_filesz as usize])
                        .map_err(|_| ModuleLoadError::BrokenModuleInfo)?
                        .to_owned();
            }
            // Unknown or unhandled type. Do nothing.
            _ => (),
        }
    }

    // Convert addresses to offsets in the binary so we can read their values.
    let fix_addr = |opt: &mut Option<_>| {
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
    fix_addr(&mut dt_hash);

    let strtab = &data[dt_strtab.unwrap() as usize..][..dt_strsz.unwrap() as usize];

    // Load symbol table. To get the size of it, we need to look at the DT_HASH tag.
    let symtab_len = bytemuck::try_from_bytes::<ElfHashTable>(
        &data[dt_hash.unwrap() as usize..][..size_of::<ElfHashTable>()],
    )
    .map_err(|_| ModuleLoadError::InvalidData)?
    .nchain as usize;

    let symtab: &[ElfSym] = bytemuck::try_cast_slice(
        &data[dt_symtab.unwrap() as usize..][..symtab_len * size_of::<ElfSym>()],
    )
    .map_err(|_| ModuleLoadError::InvalidData)?;

    // Handle relocations.
    let do_reloc = |addr: _, size: _| -> _ {
        let relas: &[ElfRela] = bytemuck::try_cast_slice(&data[addr as usize..][..size as usize])
            .map_err(|_| ModuleLoadError::InvalidData)?;

        for rela in relas {
            // The symbol index is stored in the upper 32 bits.
            let sym = (rela.r_info >> 32) as u32;
            let typ = (rela.r_info & 0xFFFF_FFFF) as u32;

            let symbol = symtab[sym as usize];

            // The address where to write the relocated address to.
            let location = (load_base + rela.r_offset as usize) as *mut usize;

            // Do the relocation.
            match typ {
                elf::R_COMMON_NONE => (),
                elf::R_COMMON_64 | elf::R_COMMON_GLOB_DAT | elf::R_COMMON_JUMP_SLOT => {
                    // Check if this symbol has an associated section.
                    // If it does not, we need to look the symbol up in our own list.
                    let resolved = if symbol.st_shndx == 0 {
                        // Get the symbol name.
                        let name = CStr::from_bytes_until_nul(&strtab[symbol.st_name as usize..])
                            .map_err(|_| ModuleLoadError::InvalidData)?
                            .to_str()
                            .map_err(|_| ModuleLoadError::InvalidData)?;
                        let kernel_symbol = SYMBOL_TABLE
                            .lock()
                            .get(name)
                            .ok_or(ModuleLoadError::SymbolNotFound)?
                            .0;

                        kernel_symbol.st_value as usize
                    } else {
                        load_base + symbol.st_value as usize
                    };

                    unsafe {
                        *location = resolved + rela.r_addend as usize;
                    }
                }
                elf::R_COMMON_RELATIVE => unsafe {
                    *location = load_base + rela.r_addend as usize;
                },
                _ => return Err(ModuleLoadError::UnsupportedRelocation),
            }
        }
        Ok(())
    };

    if let Some(addr) = dt_rela {
        do_reloc(addr, dt_relasz.ok_or(ModuleLoadError::InvalidData)?)?;
    }
    if let Some(addr) = dt_jmprel {
        do_reloc(addr, dt_pltrelsz.ok_or(ModuleLoadError::InvalidData)?)?;
    }

    // Finally, remap everything so the permissions are as described.
    for (_, virt, length, flags) in &info.mappings {
        let length = align_up(*length, arch::virt::get_page_size(VmLevel::L1));
        let page_table = PageTable::get_kernel();
        for page in (0..length).step_by(arch::virt::get_page_size(VmLevel::L1)) {
            page_table
                .remap_single::<KernelAlloc>(*virt + page, *flags, VmLevel::L1)
                .map_err(|_| ModuleLoadError::AllocFailed)?;
        }
    }

    // Register newly added symbols for dependencies.
    for symbol in symtab {
        // TODO: Add symbols
    }

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
        .filter(|x| *x != "menix.kso")
        .collect::<Vec<_>>();

    log!("Loaded module \"{}\":", name);
    log!("  Base Address | {:#x}", load_base);
    log!("  Description  | {}", info.description);
    log!("  Version      | {}", info.version);
    log!("  Author(s)    | {}", info.author);
    log!("  Dependencies | {:?}", dependencies);

    // TODO: Load dependencies

    // TODO: Call init array

    // Call the entry.
    unsafe {
        info.entry = Some(core::mem::transmute(load_base + elf_hdr.e_entry as usize));

        if let Some(entry_point) = info.entry {
            (entry_point)();
        }
    }

    MODULE_TABLE.lock().insert(name.to_owned(), info);

    return Ok(());
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_string_section {
    (expanded $(#[$meta:meta])* $name:ident $src:expr) => {
        #[doc(hidden)]
        #[used]
        $(#[$meta])*
        static $name: [u8; $src.len()] = {
            let mut buf = [0u8; $src.len()];
            let src = $src;
            let mut idx = 0;
            while idx < src.len() {
                buf[idx] = src[idx];
                idx += 1;
            }
            buf
        };
    };
    ($($(#[$meta:meta])* static $name:ident = $str:expr;)*) => {
        $(
            $crate::define_string_section!(expanded $(#[$meta])* $name $str.as_bytes());
        )*
    };
}

#[macro_export]
macro_rules! module {
    ($desc: expr, $author: expr, $entry: ident) => {
        $crate::define_string_section! {
            #[unsafe(link_section = ".mod.version")]
            static MODULE_VERSION = env!("CARGO_PKG_VERSION");

            #[unsafe(link_section = ".mod.desc")]
            static MODULE_DESC = $desc;

            #[unsafe(link_section = ".mod.author")]
            static MODULE_AUTHOR = $author;
        }

        #[unsafe(no_mangle)]
        unsafe extern "C" fn _start() {
            $entry();
        }
    };
}
