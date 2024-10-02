mod apic;
mod asm;
mod consts;
mod elf;
mod gdt;
mod idt;
mod pm;
mod vm;

use super::{CommonArch, CommonCpu};
use crate::{
    boot::BootInfo,
    memory::{self, pm::CommonPhysManager, vm::CommonVirtManager},
    thread::thread::Thread,
};
use alloc::{sync::Arc, vec::Vec};
use core::{arch::asm, ptr::null_mut};
use gdt::GDT_TABLE;
use idt::IDT_TABLE;
pub use pm::PhysManager;
pub use vm::VirtManager;

pub struct Arch;
impl CommonArch for Arch {
    unsafe fn early_init(info: &BootInfo) {
        GDT_TABLE.load();
        IDT_TABLE.load();
        pm::PhysManager::init(info);
        vm::VirtManager::init(info);
        memory::slab::init();
    }

    unsafe fn init(info: &BootInfo) {
        // Allocate memory to hold the processor specific data.
        let mut cpu_data = Vec::with_capacity(info.smp_info.processors.len());

        // Copy processor information.
        for (i, cpu) in info.smp_info.processors.iter().enumerate() {
            let c = Cpu {
                id: i,
                thread: None,
            };
            cpu_data.push(c);
        }

        let mut slice = cpu_data.into_boxed_slice();
        PER_CPU_DATA = slice.as_mut_ptr();
    }

    unsafe fn init_cpu(cpu: &Cpu, boot_cpu: &Cpu) {
        // TODO: Set KERNEL_GSBASE to the raw address of the Vec
        todo!()
    }

    #[cfg(feature = "smp")]
    fn current_cpu() -> &'static mut Cpu {
        unsafe {
            let idx: usize;
            // The Cpu struct starts at KERNEL_GSBASE:0
            // Since we can't "directly" access the base address, just get the first field (Cpu.id)
            // and use that to index into the CPU array.
            asm!(
                "mov {0}, gs:[{1}]",
                out(reg) idx,
                const core::mem::offset_of!(Cpu, id),
                options(nostack, preserves_flags),
            );
            return PER_CPU_DATA.add(idx).as_mut().unwrap();
        }
    }

    #[cfg(not(feature = "smp"))]
    fn current_cpu() -> &'static mut Cpu {
        unsafe {
            return PER_CPU_DATA.get_mut(0).unwrap();
        }
    }
}

/// Processor-local information.
#[repr(C, align(4096))]
pub struct Cpu {
    id: usize,
    thread: Option<Arc<Thread>>,
}

static mut PER_CPU_DATA: *mut Cpu = null_mut();

impl CommonCpu for Cpu {
    fn id(&self) -> usize {
        self.id
    }

    fn thread(&self) -> &Option<Arc<Thread>> {
        &self.thread
    }

    fn set_thread(&mut self, thread: &Arc<Thread>) {
        self.thread = Some(Arc::clone(thread));
    }
}

/// Registers which are saved and restored during a context switch or interrupt.
#[derive(Clone, Debug, Default)]
#[allow(unused)]
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
pub type PhysAddr = u64;

/// Represents a virtual address. It can't be directly read from or written to.
/// Note: Not the same as a pointer. A `VirtAddr` might point into another
/// process's memory that is not mapped in the kernel.
pub type VirtAddr = u64;
