use crate::{
    arch::VirtAddr,
    generic::{alloc::virt, elf},
};
use alloc::vec::Vec;
use core::ffi::CStr;
use spin::Mutex;

#[repr(C, packed)]
pub struct Module {
    pub init: fn() -> i32,
    pub exit: Option<fn() -> i32>,
    pub name: [u8; 64],
    pub description: [u8; 64],
    pub num_dependencies: usize,
}

static SYMBOL_TABLE: Mutex<Vec<elf::ElfSym>> = Mutex::new(Vec::new());

/// Sets up the module system.
pub fn init() {
    let dynsym_start = &raw const virt::LD_DYNSYM_START as *const elf::ElfSym;
    let dynsym_end = &raw const virt::LD_DYNSYM_END;
    let dynstr_start = &raw const virt::LD_DYNSTR_START;
    let dynstr_end = &raw const virt::LD_DYNSTR_END;

    // Add all kernel symbols to a table so we can perform dynamic linking.
    {
        let mut symbol_table = SYMBOL_TABLE.lock();
        for symbol_idx in 0..(dynsym_end as VirtAddr - dynsym_start as VirtAddr) {
            let symbol = unsafe { dynsym_start.add(symbol_idx) };
            let s = unsafe { CStr::from_ptr(dynstr_start.add((*symbol).st_name as usize)) };
            if let Ok(x) = s.to_str() {
                if x.len() > 0 {
                    // TODO: Add symbol
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
