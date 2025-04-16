// Per-CPU data structures.

use super::{memory::VirtAddr, sched::thread::Thread};
use crate::arch::{self, cpu::ArchPerCpu};
use alloc::{boxed::Box, sync::Arc};
use core::{
    ptr::null_mut,
    sync::atomic::{AtomicUsize, Ordering},
};
use spin::RwLock;

/// Processor-local information.
#[derive(Debug)]
pub struct PerCpu {
    /// A pointer to this structure.
    pub ptr: *mut PerCpu,
    /// The ID of this CPU.
    pub id: usize,
    /// Stack pointer for kernel mode. Only used for task switching.
    pub kernel_stack: VirtAddr,
    /// Stack pointer for user mode.
    pub user_stack: VirtAddr,
    /// Whether this CPU is online.
    pub online: bool,
    /// Whether this CPU is present.
    pub present: bool,
    /// Current thread running on this CPU.
    pub thread: Option<Arc<RwLock<Thread>>>,
    /// Architecture-specific fields.
    pub arch: ArchPerCpu,
}

impl PerCpu {
    /// Initializes the CPU which is being used to boot (This CPU).
    pub fn setup_bsp() {
        let mut cpu = Box::leak(Box::new(PerCpu {
            ptr: null_mut(),
            id: 0,
            kernel_stack: VirtAddr(0),
            user_stack: VirtAddr(0),
            online: true,
            present: true,
            thread: None,
            arch: ArchPerCpu::new(),
        }));

        cpu.ptr = cpu;

        // Some fields are not generic, initialize them too.
        cpu.arch_setup_cpu();
    }
}
