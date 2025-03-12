// Per-CPU data structures.

use super::{schedule::Scheduler, thread::Thread};
use crate::{
    arch::{self, VirtAddr, percpu::ArchPerCpu},
    generic::{
        phys::{self, PhysManager},
        virt,
    },
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};
use spin::Mutex;

/// Processor-local information.
#[repr(C, align(0x10))]
#[derive(Debug)]
pub struct PerCpu {
    /// A pointer to this structure.
    pub ptr: *mut PerCpu,
    /// The ID of this CPU.
    pub id: usize,
    /// Current thread running on this CPU.
    pub scheduler: AtomicPtr<Scheduler>,
    /// Stack pointer for kernel mode.
    pub kernel_stack: *mut u8,
    /// Stack pointer for user mode. Don't use directly.
    pub user_stack: VirtAddr,
    /// Amount of ticks the current thread has been running for.
    pub ticks_active: AtomicUsize,
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

    // TODO: Allocate a kernel stack.

    let mut cpu = Box::leak(Box::new(PerCpu {
        id: next_id,
        ptr: null_mut(),
        scheduler: AtomicPtr::new(null_mut()),
        kernel_stack: null_mut(),
        user_stack: 0,
        ticks_active: AtomicUsize::new(0),
        enabled: true,
        arch: ArchPerCpu::new(),
    }));

    cpu.ptr = cpu;

    // Some fields are not generic, initialize them too.
    PerCpu::arch_setup_cpu(cpu);

    print!("percpu: CPU {} is active.\n", next_id);
}
