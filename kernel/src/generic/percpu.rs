// Per-CPU data structures.

use super::task::Task;
use crate::arch::{self, VirtAddr, percpu::ArchPerCpu};
use alloc::{sync::Arc, vec::Vec};
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

static PER_CPU_DATA: Mutex<Vec<PerCpu>> = Mutex::new(Vec::new());

/// Initializes the current processor.
pub fn setup_cpu() {
    let mut per_cpu = PER_CPU_DATA.lock();

    let next_id = per_cpu.len();
    print!("percpu: Initializing CPU {}.\n", next_id);

    let mut cpu = PerCpu {
        id: next_id,
        kernel_stack: 0,
        user_stack: 0,
        thread: None,
        ticks_active: 0,
        enabled: true,
        arch: ArchPerCpu::new(),
    };

    // Some fields are not generic, initialize them too.
    arch::percpu::setup_cpu(&mut cpu);

    per_cpu.push(cpu);
    print!(
        "percpu: CPU {} is active. Total {} CPUs.\n",
        next_id,
        per_cpu.len()
    );
}
