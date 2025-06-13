use super::internal;
use crate::generic::percpu::CpuData;

/// Returns the value of the frame pointer register.
pub fn get_frame_pointer() -> usize {
    internal::core::get_frame_pointer()
}

/// Returns the per-CPU data of this CPU.
pub fn get_per_cpu() -> *mut CpuData {
    internal::core::get_per_cpu()
}

/// Stop execution on this CPU.
pub fn halt() -> ! {
    internal::core::halt()
}
