#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "loongarch64"
))]
pub mod acpi;

pub mod openfw;

pub mod pci;

use super::{boot::BootInfo, percpu::CpuData};

#[deny(dead_code)]
pub fn init() {
    let info = BootInfo::get();

    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    ))]
    if info.command_line.get_bool("acpi").unwrap_or(true) {
        acpi::init();
    }

    // TODO: OpenFirmware support.

    // Initialize BSP.
    crate::arch::core::perpare_cpu(CpuData::get());
    // TODO: Initialize other cores.

    // Initalize system buses.
    if info.command_line.get_bool("pci").unwrap_or(true) {
        pci::init();
    }
}
