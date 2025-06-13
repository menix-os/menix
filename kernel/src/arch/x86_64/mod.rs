mod asm;
mod consts;
pub mod core;
pub mod irq;
pub mod sched;
pub mod system;
pub mod virt;

use crate::generic::{irq::IrqHandler, util::mutex::Mutex};
use ::core::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize};
use alloc::boxed::Box;
use system::gdt::{Gdt, TaskStateSegment};

#[derive(Debug)]
#[repr(C)]
pub struct ArchPerCpu {
    /// Processor local Global Descriptor Table.
    /// The GDT refers to a different TSS every time, so unlike the IDT it has to exist for each processor.
    pub gdt: Mutex<Gdt>,
    pub tss: Mutex<TaskStateSegment>,
    /// IRQ mappings.
    pub irq_handlers: Mutex<[Option<Box<dyn IrqHandler>>; 256]>,
    /// A map of ISRs to IRQs.
    pub irq_map: Mutex<[usize; 256]>,
    /// Size of the FPU.
    pub fpu_size: AtomicUsize,
    /// Function called to save the FPU context.
    pub fpu_save: AtomicPtr<unsafe fn(memory: *mut u8)>,
    /// Function called to restore the FPU context.
    pub fpu_restore: AtomicPtr<unsafe fn(memory: *const u8)>,
    /// If this CPU supports the STAC/CLAC instructions.
    pub can_smap: AtomicBool,
}

per_cpu!(
    pub(crate) static ARCH_DATA: ArchPerCpu = ArchPerCpu {
        gdt: Mutex::new(Gdt::new()),
        tss: Mutex::new(TaskStateSegment::new()),
        irq_handlers: Mutex::new([const { None }; 256]),
        irq_map: Mutex::new([const { 0 }; 256]),
        fpu_size: AtomicUsize::new(512),
        fpu_save: AtomicPtr::new(asm::fxsave as _),
        fpu_restore: AtomicPtr::new(asm::fxrstor as _),
        can_smap: AtomicBool::new(false),
    };
);
