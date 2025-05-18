#[cfg(feature = "acpi")]
pub mod acpi;

#[cfg(feature = "openfw")]
pub mod openfw;

#[cfg(feature = "pci")]
pub mod pci;

use super::{boot::BootInfo, percpu::CpuData};

#[deny(dead_code)]
pub fn init() {
    let info = BootInfo::get();

    #[cfg(feature = "acpi")]
    if info.command_line.get_bool("acpi").unwrap_or(true) {
        acpi::init();
    }

    // TODO: OpenFirmware support.

    // Initialize BSP.
    crate::arch::core::perpare_cpu(CpuData::get());
    // TODO: Initialize other cores.

    // Initalize system busses.

    #[cfg(feature = "pci")]
    pci::init();
}
