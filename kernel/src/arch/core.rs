use super::internal;
use crate::generic::{percpu::CpuData, posix::errno::EResult};

pub fn setup_bsp() {
    internal::core::setup_bsp()
}

/// Returns the value of the frame pointer register.
pub fn get_frame_pointer() -> usize {
    internal::core::get_frame_pointer()
}

/// Returns the per-CPU data of this CPU.
pub fn get_per_cpu() -> *mut CpuData {
    internal::core::get_per_cpu()
}

/// Performs some CPU-dependent operation.
pub fn archctl(cmd: usize, arg: usize) -> EResult<usize> {
    internal::core::archctl(cmd, arg)
}

/// Stop all other CPUs.
pub fn halt_others() {
    internal::core::halt_others()
}

/// Stop this CPU.
pub fn halt() -> ! {
    internal::core::halt()
}
