use super::boot::BootInfo;

#[cfg(feature = "acpi")]
pub mod acpi;

#[cfg(feature = "openfw")]
pub mod openfw;

#[deny(dead_code)]
pub fn init() {
    let info = BootInfo::get();

    #[cfg(feature = "acpi")]
    if info.command_line.get_bool("acpi").unwrap_or(true) {
        acpi::init();
    }

    // TODO: OpenFirmware support.
}
