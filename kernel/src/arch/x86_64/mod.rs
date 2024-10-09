mod apic;
mod asm;
mod consts;
mod elf;
mod gdt;
mod idt;
mod interrupts;
mod pm;
mod sched;
mod tss;
mod vm;

use super::{CommonArch, CommonContext, CommonCpu};
use crate::{
    boot::BootInfo,
    memory::{pm::CommonPhysManager, vm::CommonVirtManager},
    thread::thread::Thread,
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use consts::MSR_KERNEL_GS_BASE;
use core::{arch::asm, ptr::null_mut};
pub use pm::PhysManager;
pub use vm::VirtManager;

pub const PAGE_SIZE: usize = 0x1000;

pub struct Arch;
impl CommonArch for Arch {
    unsafe fn early_init(info: &mut BootInfo) {
        unsafe {
            gdt::load();
            idt::load();

            pm::PhysManager::init(info);
            asm!("int 0x80");
            vm::VirtManager::init(info);
        }
    }

    unsafe fn init(info: &mut BootInfo) {
        // Allocate memory to hold the processor specific data.
        let mut cpu_data = Vec::with_capacity(info.smp_info.processors.len());

        // Copy processor information.
        for (i, cpu) in info.smp_info.processors.iter().enumerate() {
            // TODO
            let c = Cpu {
                id: i,
                lapic_id: cpu.lapic_id,
                thread: None,
            };
            cpu_data.push(c);
        }

        let mut buffer = cpu_data.into_boxed_slice();
        unsafe {
            PER_CPU_DATA = buffer.as_mut_ptr();
            // Intentionally leak memory so it doesn't ever get freed.
            Box::leak(buffer);
        }
    }

    unsafe fn init_cpu(cpu: &Cpu, boot_cpu: &Cpu) {
        unsafe {
            asm::wrmsr(MSR_KERNEL_GS_BASE, PER_CPU_DATA.add(cpu.id) as u64);
        }
        // TODO
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
            return PER_CPU_DATA.as_mut().unwrap();
        }
    }
}

/// Processor-local information.
#[repr(C, align(0x10))]
#[derive(Clone, Debug)]
pub struct Cpu {
    id: usize,
    lapic_id: usize,
    thread: Option<Arc<Thread>>,
}

/// Fixed buffer for CPU-local storage. Stores important CPU state information.
/// Never to be used directly! Always use Arch::current_cpu() instead.
static mut PER_CPU_DATA: *mut Cpu = null_mut();

impl CommonCpu for Cpu {
    fn id(&self) -> usize {
        self.id
    }

    fn thread(&self) -> Option<&Arc<Thread>> {
        match &self.thread {
            Some(x) => Some(x),
            None => None,
        }
    }

    fn set_thread(&mut self, thread: &Arc<Thread>) {
        self.thread = Some(Arc::clone(thread));
    }
}

/// Registers which are saved and restored during a context switch or interrupt.
#[repr(C, packed)]
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
    // Pushed onto the stack by the interrupt handler stubs.
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

impl CommonContext for Context {}

/// Represents a physical address. It can't be directly read from or written to.
pub type PhysAddr = u64;

/// Represents a virtual address. It can't be directly read from or written to.
/// Note: Not the same as a pointer. A `VirtAddr` might point into another
/// process's memory that is not mapped in the kernel.
pub type VirtAddr = u64;
