mod asm;
mod consts;
mod gdt;
mod idt;
pub mod init;
pub mod interrupts;
mod kvmclock;
pub mod percpu;
pub mod schedule;
mod serial;
mod tsc;
mod tss;
pub mod virt;

pub type PhysAddr = usize;
pub type VirtAddr = usize;
