mod asm;
mod consts;
pub mod core;
pub mod irq;
pub mod sched;
pub mod system;
pub mod virt;

use crate::generic::{
    irq::IrqHandlerKind,
    util::{mutex::Mutex, once::Once},
};
use ::core::sync::atomic::AtomicBool;
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
    pub fpu_size: Once<usize>,
    /// Function called to save the FPU context.
    pub fpu_save: Once<unsafe fn(*mut u8)>,
    /// Function called to restore the FPU context.
    pub fpu_restore: Once<unsafe fn(*const u8)>,
    /// If this CPU supports the STAC/CLAC instructions.
    pub can_smap: AtomicBool,
}

per_cpu!(
    pub(crate) static ARCH_DATA: ArchPerCpu = ArchPerCpu {
        gdt: Mutex::new(Gdt::new()),
        tss: Mutex::new(TaskStateSegment::new()),
        irq_handlers: Mutex::new([const { IrqHandlerKind::None }; 256]),
        fpu_size: Once::new(),
        fpu_save: Once::new(),
        fpu_restore: Once::new(),
        can_smap: AtomicBool::new(false),
    };
);
