// Per-CPU data structures.

use super::task::Task;
use crate::arch::{self, VirtAddr, percpu::ArchPerCpu};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;

/// Processor-local information.
#[repr(C, align(0x10))]
#[derive(Debug)]
pub struct PerCpu {
    /// Unique identifier of this CPU.
    pub id: usize,
    /// Stack pointer for kernel mode.
    pub kernel_stack: VirtAddr,
    /// Stack pointer for user mode.
    pub user_stack: VirtAddr,
    /// Current thread running on this CPU.
    pub thread: Option<Arc<Task>>,
    /// Amount of ticks the current thread has been running for.
    pub ticks_active: usize,
    /// Whether this CPU is enabled.
    pub enabled: bool,

    /// Architecture-specific fields.
    pub arch: ArchPerCpu,
}

static CPU_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Initializes the current processor.
pub fn setup_cpu() {
    let next_id = CPU_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    print!("percpu: Initializing CPU {}.\n", next_id);

    let mut cpu = Box::new(PerCpu {
        id: next_id,
        kernel_stack: 0,
        user_stack: 0,
        thread: None,
        ticks_active: 0,
        enabled: true,
        arch: ArchPerCpu::new(),
    });

    // Some fields are not generic, initialize them too.
    arch::percpu::setup_cpu(cpu);

    print!("percpu: CPU {} is active.\n", next_id);
}
