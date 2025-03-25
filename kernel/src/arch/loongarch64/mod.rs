pub mod init;
pub mod interrupts;
pub mod percpu;
pub mod schedule;
pub mod virt;

pub type PhysAddr = usize;
pub type VirtAddr = usize;
