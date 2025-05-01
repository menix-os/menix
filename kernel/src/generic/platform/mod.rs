use super::boot::BootInfo;

#[cfg(feature = "acpi")]
pub mod acpi;

#[cfg(feature = "openfw")]
pub mod openfw;

#[cfg(feature = "pci")]
pub mod pci;

#[deny(dead_code)]
pub fn init() {
    let info = BootInfo::get();

    #[cfg(feature = "acpi")]
    if info.command_line.get_bool("acpi").unwrap_or(true) {
        acpi::init();
    }

    // TODO: OpenFirmware support.

    #[cfg(feature = "pci")]
    pci::init();
}
