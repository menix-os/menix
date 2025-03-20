use crate::{
    arch::VirtAddr,
    generic::{elf, virt},
};
use alloc::{borrow::ToOwned, collections::btree_map::BTreeMap, string::String};
use core::{ffi::CStr, slice};
use spin::Mutex;

#[repr(C, packed)]
pub struct Module {
    pub init: fn() -> i32,
    pub exit: Option<fn() -> i32>,
    pub name: [u8; 64],
    pub description: [u8; 64],
    pub num_dependencies: usize,
}

static SYMBOL_TABLE: Mutex<BTreeMap<String, elf::ElfSym>> = Mutex::new(BTreeMap::new());

/// Sets up the module system.
pub fn init() {
    let dynsym_start = &raw const virt::LD_DYNSYM_START;
    let dynsym_end = &raw const virt::LD_DYNSYM_END;
    let dynstr_start = &raw const virt::LD_DYNSTR_START;
    let dynstr_end = &raw const virt::LD_DYNSTR_END;

    // Add all kernel symbols to a table so we can perform dynamic linking.
    {
        let mut symbol_table = SYMBOL_TABLE.lock();
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

        let mut idx = 0;
        for sym in symbols {
            let name = unsafe { CStr::from_bytes_until_nul(&strings[(*sym).st_name as usize..]) };
            if let Ok(x) = name {
                if let Ok(s) = x.to_str() {
                    if s.len() > 0 {
                        assert!(symbol_table.insert(s.to_owned(), *sym).is_none());
                    }
                }
            }
        }
    }

    print!(
        "module: Registered {} kernel symbols.\n",
        SYMBOL_TABLE.lock().len()
    );
}

/// Loads a module from file.
pub fn load() {}
