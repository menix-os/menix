use super::internal;
use crate::generic::percpu::CpuData;

/// Sets up the Bootstrap Processor.
pub unsafe fn setup_bsp() {
    unsafe { internal::core::setup_bsp() };
}

/// Returns the per-CPU data of this CPU.
pub fn get_per_cpu() -> *mut CpuData {
    internal::core::get_per_cpu()
}

/// Tests and enables all supported features on the current CPU.
pub fn perpare_cpu(context: &mut CpuData) {
    internal::core::perpare_cpu(context);
}

/// Stop execution on this CPU.
pub fn halt() -> ! {
    internal::core::halt()
}
