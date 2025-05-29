//! The global panic handler. Panics are usually non-recoverable error cases.
//! It halts all kernel function and prints some info about the machine state.

use super::log::GLOBAL_LOGGERS;
use crate::{arch, generic::elf::ElfAddr};
use core::panic::PanicInfo;

#[repr(C)]
#[derive(Debug)]
struct StackFrame {
    prev: *const StackFrame,
    return_addr: *const (),
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // TODO: Do this on all CPUs.
    unsafe { arch::irq::set_irq_state(false) };

    // Force unlock output in cases like panics during printing.
    unsafe { GLOBAL_LOGGERS.force_unlock(true) };

    error!("Kernel panic - Environment is unsound!");
    if let Some(location) = info.location() {
        error!("at {}", location);
    }
    error!("{}", info.message());
    error!("---------------------");

    {
        let modules = &*super::module::MODULE_TABLE.lock();
        if modules.len() == 0 {
            error!("No linked modules");
        } else {
            error!("{} linked module(s):", modules.len());
            for (name, module) in modules.iter() {
                error!("{}:", name);
                for (_, virt, length, flags) in &module.mappings {
                    error!(
                        "    {:#x} - {:#x}, {:?}",
                        virt.value(),
                        virt.value() + length,
                        flags
                    );
                }
            }
        }
    }

    // Do a stack trace.
    unsafe {
        let table = &*super::module::SYMBOL_TABLE.lock();

        let mut fp = arch::core::get_frame_pointer() as *const StackFrame;
        while (fp as usize) > 1usize << (usize::BITS - 1) {
            let addr = (*fp).return_addr as ElfAddr;
            let symbol = table.iter().find(|(_, (sym, _))| {
                (addr >= sym.st_value) && (addr <= (sym.st_value + sym.st_size))
            });
            let (name, offset) = symbol
                .map(|(name, (sym, _))| (name.as_str(), addr - sym.st_value))
                .unwrap_or(("???", 0));

            error!(
                "{:#x} <{:#} + {:#x}>",
                addr as u64,
                rustc_demangle::demangle(name),
                offset
            );
            fp = (*fp).prev;
        }
    }

    loop {}
}
