mod apic;
mod asm;
mod consts;
pub mod firmware;
mod gdt;
mod idt;
pub mod init;
pub mod irq;
mod kvmclock;
pub mod page;
pub mod percpu;
mod serial;
mod tsc;

pub type PhysAddr = usize;
pub type VirtAddr = usize;
