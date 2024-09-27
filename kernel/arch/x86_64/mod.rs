mod asm;
mod consts;
mod elf;
mod gdt;
mod idt;

use crate::boot::BootInfo;
use gdt::GDT_TABLE;
use idt::IDT_TABLE;

pub struct Arch {}

impl Arch {
    /// Initializes serial I/O, loads the kernel's GDT/IDT and starts the memory allocator.
    pub fn early_init(_info: &BootInfo) {
        GDT_TABLE.load();
        IDT_TABLE.load();
    }

    /// Prepares all available processors
    pub fn init(info: &BootInfo) {
        // TODO
        todo!()
    }
}

/// Processor-local information. Stores its own index and processor state.
pub struct Processor {}

/// Registers which are saved and restored during a context switch or interrupt.
#[derive(Clone, Debug, Default)]
pub struct Context {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    rdx: u64,
    rcx: u64,
    rbx: u64,
    rax: u64,

    // Pushed onto the stack by the interrupt handler stub.
    core: u64,
    isr: u64,

    // Pushed onto the stack by the CPU if the interrupt has an error code.
    error: u64,
    // Pushed onto the stack by the CPU during an interrupt.
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

/// Represents a physical address. It can't be directly read from or written to.
#[derive(Default)]
pub struct PhysAddr(usize);

/// Represents a virtual address. It can't be directly read from or written to.
/// Note: Not the same as a pointer. A `VirtAddr` might point into another
/// process's memory that is not mapped in the kernel.
#[derive(Default)]
pub struct VirtAddr(usize);
