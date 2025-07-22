pub mod config;
pub mod device;
pub mod driver;

use crate::{
    generic::util::once::Once,
    system::pci::{config::scan_config_space, device::PciDevice},
};
use alloc::vec::Vec;

#[derive(Debug)]
pub enum PciError {
    Unknown,
    DriverAlreadyExists,
}

/// Initializes the PCI subsystem.
#[initgraph::task(name = "system.pci")]
#[cfg_attr(
    feature = "acpi",
    initgraph::task(depends = [super::acpi::INIT_STAGE])
)]
pub fn PCI_STAGE() {
    log!("Initializing the PCI subsystem");

    unsafe { BUSES.init(scan_config_space()) };
}
