use spin::RwLock;

use crate::boot::BootInfo;

#[cfg(feature = "acpi")]
pub mod acpi;

#[cfg(feature = "openfw")]
pub mod openfw;

/// Initializes the firmware interface.
pub fn init() {
    print!("fw: Initializing firmware.\n");

    #[cfg(feature = "acpi")]
    if let Some(rsdp) = BootInfo::get().rsdp_addr {
        acpi::init(rsdp);
    }
}
