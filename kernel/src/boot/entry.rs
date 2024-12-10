use crate::fs::vfs;

/// Main initialization function after kernel_boot.
pub fn kernel_main() -> ! {
    vfs::init();
    todo!();
}
