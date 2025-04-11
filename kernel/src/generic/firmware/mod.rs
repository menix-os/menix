use spin::RwLock;

use crate::boot::BootInfo;

pub mod acpi;
pub mod fdt;

/// Initializes the firmware interface.
pub fn init() {
    print!("fw: Initializing firmware.\n");
    if let Some(rsdp) = BootInfo::get().rsdp_addr {
        acpi::init(rsdp);
    }
}
