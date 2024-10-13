use crate::fs::vfs;

/// Main initialization function after kernel_boot.
/// At this point, everything is already run via the scheduler.
pub fn kernel_main() -> ! {
    vfs::init();
    todo!();
}
