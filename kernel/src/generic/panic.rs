//! The global panic handler. Panics are usually non-recoverable error cases.
//! It halts all kernel function and prints some info about the machine state.

use super::log::GLOBAL_LOGGERS;
use crate::{
    arch,
    generic::{
        memory::{VirtAddr, virt::PageTable},
        vfs::exec::elf::ElfAddr,
    },
};
use core::panic::PanicInfo;

#[repr(C)]
#[derive(Debug)]
struct StackFrame {
    prev: *const StackFrame,
    return_addr: *const (),
}

macro_rules! log_panic {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = GLOBAL_LOGGERS.lock();
        _ = writer.write_fmt(format_args!("[    !!!!    ] "));
        _ = writer.write_fmt(format_args!("\x1b[1;31m"));
        _ = writer.write_fmt(format_args!($($arg)*));
        _ = writer.write_fmt(format_args!("\x1b[0m\n"));
    });
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // TODO: Do this on all CPUs.
    unsafe { arch::irq::set_irq_state(false) };

    // Force unlock output in cases like panics during printing.
    unsafe { GLOBAL_LOGGERS.force_unlock(false) };

    // We write directly to the loggers because something might've happened to the timers.
    log_panic!("Kernel panic - Environment is unsound!");
    if let Some(location) = info.location() {
        log_panic!("at {}", location);
    }
    log_panic!("{}", info.message());

    {
        log_panic!("----------");
        let modules = super::module::MODULE_TABLE.lock();
        match modules.len() {
            0 => log_panic!("No linked modules"),
            len => {
                log_panic!("{} linked module(s):", len);
                for (name, module) in modules.iter() {
                    log_panic!("{}:", name);
                    for (_, virt, length, flags) in &module.mappings {
                        log_panic!(
                            "    {:#x} - {:#x}, {:?}",
                            virt.value(),
                            virt.value() + length,
                            flags
                        );
                    }
                }
            }
        }
    }

    // Do a stack trace.
    unsafe {
        super::module::SYMBOL_TABLE.force_unlock(false);

        let table = &*super::module::SYMBOL_TABLE.lock();

        log_panic!("----------");
        log_panic!("Stack trace (most recent call first):");

        let mut fp = arch::core::get_frame_pointer() as *const StackFrame;
        let kernel_map = PageTable::get_kernel();

        while kernel_map.is_mapped(VirtAddr::from(fp)) {
            let addr = (*fp).return_addr as ElfAddr;
            let symbol = table.iter().find(|(_, (sym, _))| {
                (addr >= sym.st_value) && (addr <= (sym.st_value + sym.st_size))
            });
            let (name, offset) = symbol
                .map(|(name, (sym, _))| (name.as_str(), addr - sym.st_value))
                .unwrap_or(("???", 0));

            if addr == 0 {
                break;
            }

            log_panic!(
                "{:#x} <{:#} + {:#x}>",
                addr as u64,
                rustc_demangle::demangle(name),
                offset
            );
            fp = (*fp).prev;
        }
    }

    log_panic!("----------");
    log_panic!("End of panic message");

    loop {
        unsafe { arch::irq::set_irq_state(false) };
        arch::irq::wait_for_irq();
    }
}
