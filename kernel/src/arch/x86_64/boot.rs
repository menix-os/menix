use crate::generic::{self, boot};

use super::{apic, asm::interrupt_disable, idt, serial};

/// Entry point for x86_64.
#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    unsafe { interrupt_disable() };
    apic::disable_legacy_pic();
    serial::init();
    idt::init();

    crate::main();
}
