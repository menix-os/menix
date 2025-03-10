use crate::{boot::BootInfo, generic::percpu::PerCpu};
use core::arch::asm;

pub fn early_init() {
    super::serial::init();
    super::idt::init();
}

pub fn init(info: &mut BootInfo) {}

/// Returns the ID of this CPU.
pub fn current_cpu() -> usize {
    unsafe {
        let cpu: usize;
        asm!(
            "mov {0}, gs:[0]",
            out(reg) cpu,
            options(nostack, preserves_flags),
        );
        return cpu;
    }
}
