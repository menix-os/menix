pub mod asm;
pub mod consts;
pub mod gdt;
pub mod idt;
pub mod init;
pub mod interrupts;
pub mod percpu;
pub mod schedule;
pub mod serial;
pub mod tss;
pub mod virt;

pub type PhysAddr = usize;
pub type VirtAddr = usize;
