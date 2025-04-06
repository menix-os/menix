use crate::{
    arch::VirtAddr,
    generic::{elf, memory::virt},
};
use alloc::{borrow::ToOwned, collections::btree_map::BTreeMap, string::String};
use core::{ffi::CStr, ptr::null, slice};
use spin::Mutex;

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
            let name = unsafe { CStr::from_bytes_until_nul(&strings[sym.st_name as usize..]) };
            if let Ok(x) = name {
                if let Ok(s) = x.to_str() {
                    if !s.is_empty() {
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

/// Loads a module from an ELF in memory.
pub fn load() {}

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
