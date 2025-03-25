use alloc::{boxed::Box, sync::Arc};
use core::{arch::asm, ffi::CStr, mem::offset_of, ptr::null_mut};
use spin::Mutex;

use crate::generic::{percpu::PerCpu, sched::thread::Thread};

#[derive(Debug)]
#[repr(C)]
pub struct ArchPerCpu {}

impl Default for ArchPerCpu {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchPerCpu {
    pub fn new() -> Self {
        Self {}
    }
}

impl PerCpu {
    /// Returns the per-CPU data of this CPU.
    /// # Safety
    /// Accessing this data directly is inherently unsafe without first disabling preemption!
    pub unsafe fn get_per_cpu() -> &'static mut PerCpu {
        unsafe {
            let cpu: *mut PerCpu;
            asm!(
                "csrr {cpu}, sscratch",
                cpu = out(reg) cpu,
                options(nostack, preserves_flags),
            );
            return cpu.as_mut().unwrap();
        }
    }

    /// Returns a reference to the currently running thread.
    pub fn get_thread() -> Arc<Mutex<Thread>> {
        unsafe {
            let thread: *const Arc<Mutex<Thread>>;
            asm!(
                "csrr {thread}, sscratch",
                "add {thread}, {thread}, {thread_off}",
                thread = out(reg) thread,
                thread_off = const offset_of!(PerCpu, thread),
                options(nostack, preserves_flags),
            );
            return (*thread).clone();
        }
    }

    /// Initializes architecture dependent data for the current processor.
    pub fn arch_setup_cpu(&mut self) {}
}

pub fn stop_all() -> ! {
    // TODO: This only halts the current CPU.
    loop {
        unsafe { asm!("wfi") };
    }
}
