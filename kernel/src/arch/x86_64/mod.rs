pub mod asm;
pub mod consts;
pub mod elf;
pub mod memory;
pub mod sched;
pub mod system;

use super::{CommonArch, CommonContext, CommonCpu};
use crate::boot::EarlyBootInfo;
use crate::memory::pm;
use crate::{boot::BootInfo, dbg, memory::vm::CommonVirtManager, thread::thread::Thread};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use consts::MSR_KERNEL_GS_BASE;
use core::{arch::asm, ptr::null_mut};
pub use memory::vm::PageMap;
pub use memory::vm::VirtManager;
pub use sched::Context;

pub struct Arch;
impl CommonArch for Arch {
    fn early_init(info: &mut EarlyBootInfo) {
        unsafe {
            system::serial::init();
            system::gdt::load();
            system::idt::load();

            pm::PhysManager::init(info);
            memory::vm::VirtManager::init(info);
        }
    }

    fn init(info: &mut BootInfo) {
        // Allocate memory to hold the processor specific data.
        let mut cpu_data = Vec::with_capacity(info.smp_info.processors.len());

        // Copy processor information.
        for (i, cpu) in info.smp_info.processors.iter().enumerate() {
            // TODO
            let c = Cpu {
                id: i,
                thread: None,
                kernel_stack: todo!(),
                user_stack: todo!(),
                ticks_active: todo!(),
                is_present: todo!(),
            };
            cpu_data.push(c);
        }

        let mut buffer = cpu_data.into_boxed_slice();
        unsafe {
            PER_CPU_DATA = buffer.as_mut_ptr();
        }
        // Intentionally leak memory so it doesn't ever get freed.
        Box::leak(buffer);
    }

    fn init_cpu(cpu: &Cpu, boot_cpu: &Cpu) {
        unsafe {
            asm::wrmsr(MSR_KERNEL_GS_BASE, PER_CPU_DATA.add(cpu.id) as u64);
        }
        // TODO
        todo!()
    }

    fn current_cpu() -> &'static mut Cpu {
        #[cfg(feature = "smp")]
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
        #[cfg(not(feature = "smp"))]
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
    kernel_stack: VirtAddr,
    user_stack: VirtAddr,
    thread: Option<Arc<Thread>>,
    ticks_active: usize,
    is_present: bool,
}

/// Fixed buffer for CPU-local storage. Stores important CPU state information.
/// Never to be used directly! Always use `Arch::current_cpu()` instead.
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

/// Represents a physical address. It can't be directly read from or written to.
pub type PhysAddr = u64;

/// Represents a virtual address. It can't be directly read from or written to.
/// Note: Not the same as a pointer. A `VirtAddr` might point into another
/// process's memory that is not mapped in the kernel.
pub type VirtAddr = u64;
