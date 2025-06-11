use super::internal;
use crate::generic::percpu::CpuData;

/// Sets up the Bootstrap Processor.
///
/// # Safety
/// This must only be called once.
pub unsafe fn prepare_bsp() {
    unsafe { internal::core::prepare_bsp() };
}

/// Tests and enables all supported features on a CPU that is not the BSP.
pub fn setup_ap(context: &mut CpuData) {
    internal::core::setup_ap(context);
}

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
