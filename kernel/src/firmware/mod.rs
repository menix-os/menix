use crate::boot::BootInfo;

pub mod acpi;
pub mod fdt;

/// Initializes the firmware interface.
pub fn init(info: &BootInfo) {
    print!("fw: Initializing firmware.\n");
    if let Some(rsdp) = info.rsdp_addr {
        acpi::init(rsdp);
    }
}
