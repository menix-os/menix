use super::internal;

/// Sets whether or not IRQs are enabled on this CPU.
/// Returns the old value.
/// Don't use this directly, use [`crate::generic::util::mutex::irq::IrqMutex`] instead.
///
/// # Safety
///
/// The caller must make sure that enabling interrupts is safe at this point.
pub unsafe fn set_irq_state(value: bool) -> bool {
    unsafe { internal::irq::set_irq_state(value) }
}

/// Returns true if interrupts are enabled.
pub fn get_irq_state() -> bool {
    internal::irq::get_irq_state()
}

/// Hints to stop execution on this CPU until an interrupt happens.
pub fn wait_for_irq() {
    internal::irq::wait_for_irq();
}
