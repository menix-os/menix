mod asm;
mod consts;
pub mod core;
pub mod irq;
pub mod sched;
pub mod system;
pub mod virt;

use crate::generic::{irq::IrqHandlerKind, util::mutex::Mutex};
use ::core::sync::atomic::{AtomicBool, AtomicUsize};
use system::gdt::{Gdt, TaskStateSegment};

#[derive(Debug)]
#[repr(C)]
pub struct ArchPerCpu {
    /// Processor local Global Descriptor Table.
    /// The GDT refers to a different TSS every time, so unlike the IDT it has to exist for each processor.
    pub gdt: Mutex<Gdt>,
    pub tss: Mutex<TaskStateSegment>,
    /// IRQ mappings.
    pub irq_handlers: Mutex<[IrqHandlerKind; 256]>,
    /// Size of the FPU.
    pub fpu_size: AtomicUsize,
    /// Function called to save the FPU context.
    fpu_save: AtomicUsize,
    /// Function called to restore the FPU context.
    fpu_restore: AtomicUsize,
    /// If this CPU supports the STAC/CLAC instructions.
    pub can_smap: AtomicBool,
}

per_cpu!(
    pub(crate) static ARCH_DATA: ArchPerCpu = ArchPerCpu {
        gdt: Mutex::new(Gdt::new()),
        tss: Mutex::new(TaskStateSegment::new()),
        irq_handlers: Mutex::new([const { IrqHandlerKind::None }; 256]),
        fpu_size: AtomicUsize::new(512),
        fpu_save: AtomicUsize::new(0),
        fpu_restore: AtomicUsize::new(0),
        can_smap: AtomicBool::new(false),
    };
);
