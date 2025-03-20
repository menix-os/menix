// Per-CPU data structures.

use super::sched::thread::Thread;
use crate::arch::{self, VirtAddr, percpu::ArchPerCpu};
use alloc::{boxed::Box, sync::Arc};
use core::{
    ptr::null_mut,
    sync::atomic::{AtomicUsize, Ordering},
};
use spin::RwLock;

/// Processor-local information.
#[repr(C)]
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
    /// Whether this CPU is enabled.
    pub enabled: bool,
    /// Current thread running on this CPU.
    pub thread: Option<Arc<RwLock<Thread>>>,

    /// Architecture-specific fields.
    pub arch: ArchPerCpu,
}

static CPU_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Initializes the current processor.
pub fn setup_cpu() {
    let next_id = CPU_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    print!("percpu: Initializing CPU {}.\n", next_id);

    let mut cpu = Box::leak(Box::new(PerCpu {
        ptr: null_mut(),
        id: next_id,
        kernel_stack: 0,
        user_stack: 0,
        enabled: true,
        thread: None,
        arch: ArchPerCpu::new(),
    }));

    cpu.ptr = cpu;

    // Some fields are not generic, initialize them too.
    cpu.arch_setup_cpu();

    print!("percpu: Initialized CPU {}.\n", next_id);
}

/// Stops all CPUs immediately.
pub fn stop_all() -> ! {
    arch::percpu::stop_all();
}
