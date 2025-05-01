// PCI/PCIe bus implementation

use crate::generic::boot::BootInfo;

#[cfg(not(any(feature = "acpi", feature = "openfw")))]
compile_error!("PCI needs some form of firmware support in order to work!");

pub mod device;
pub mod driver;

#[derive(Debug)]
pub enum PciError {
    Unknown,
    DriverAlreadyExists,
}

/// Initializes the PCI subsystem.
pub(crate) fn init() {
    log!("pci: Initializing the PCI subsystem");

    // First, attempt to initialize PCI using the ACPI table "MCFG".
    #[cfg(feature = "acpi")]
    if BootInfo::get()
        .command_line
        .get_bool("acpi")
        .unwrap_or(true)
    {
        log!("pci: Using ACPI to configure PCI")
    }

    // If there is no ACPI, resort to OpenFirmware.
    #[cfg(feature = "openfw")]
    {}
}
