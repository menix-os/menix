//! PCI bus implementation

use crate::generic::boot::BootInfo;

pub mod device;
pub mod driver;

#[derive(Debug)]
pub enum PciError {
    Unknown,
    DriverAlreadyExists,
}

/// Initializes the PCI subsystem.
pub(crate) fn init() {
    log!("Initializing the PCI subsystem");

    // First, attempt to initialize PCI using the ACPI table "MCFG".
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    ))]
    if BootInfo::get()
        .command_line
        .get_bool("acpi")
        .unwrap_or(true)
    {
        log!("Using ACPI to configure PCI");
        return;
    }

    // If there is no ACPI, resort to OpenFirmware.
    if BootInfo::get()
        .command_line
        .get_bool("openfw")
        .unwrap_or(true)
    {
        todo!();
    }

    panic!(
        "Unable to configure PCI without either ACPI or OpenFirmware. Reboot with `acpi=on`, `openfw=on` or `pci=off`"
    );
}
