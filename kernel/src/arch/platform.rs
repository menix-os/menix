use super::internal;

/// Calls arch-specific functions for platform initialization.
pub fn init() {
    internal::platform::init();
}
