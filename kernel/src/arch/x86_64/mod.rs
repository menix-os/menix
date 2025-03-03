use super::PerCpu;
use crate::boot::BootInfo;
use core::arch::asm;

pub mod asm;
pub mod consts;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod scheduler;
pub mod serial;
pub mod tss;
pub mod virt;

pub type PhysAddr = u64;
pub type VirtAddr = u64;

pub fn early_init() {
    serial::init();
    gdt::init();
    idt::init();
}

pub fn init(info: &mut BootInfo) {
    todo!();
}

/// Initializes a single processor.
/// `target`: The processor to initialize.
/// `boot_cpu`: The current processor.
pub fn init_cpu(target: &mut PerCpu, boot_cpu: &mut PerCpu) {
    todo!();
}

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

pub fn get_page_size() -> usize {
    0x1000
}
