mod asm;
mod consts;
pub mod core;
pub mod irq;
pub mod platform;
pub mod sched;
pub mod virt;

use crate::generic::irq::IrqHandler;
use alloc::boxed::Box;
use platform::gdt::{Gdt, TaskStateSegment};

#[derive(Debug)]
#[repr(C)]
pub struct ArchPerCpu {
    /// Processor local Global Descriptor Table.
    /// The GDT refers to a different TSS every time, so unlike the IDT it has to exist for each processor.
    pub gdt: Gdt,
    pub tss: TaskStateSegment,
    /// IRQ mappings.
    pub irq_handlers: [Option<Box<dyn IrqHandler>>; 256],
    /// A map of ISRs to IRQs.
    pub irq_map: [usize; 256],
    /// The Local APIC ID.
    pub lapic_id: u64,
    /// Size of the FPU.
    pub fpu_size: usize,
    /// Function called to save the FPU context.
    pub fpu_save: unsafe fn(memory: *mut u8),
    /// Function called to restore the FPU context.
    pub fpu_restore: unsafe fn(memory: *const u8),
    /// If this CPU supports the STAC/CLAC instructions.
    pub can_smap: bool,
}

per_cpu!(
    pub(crate) static ARCH_DATA: ArchPerCpu = ArchPerCpu {
        gdt: Gdt::new(),
        tss: TaskStateSegment::new(),
        irq_handlers: [const { None }; 256],
        irq_map: [0; 256],
        lapic_id: 0,
        fpu_size: 512,
        fpu_save: asm::fxsave,
        fpu_restore: asm::fxrstor,
        can_smap: false,
    };
);
