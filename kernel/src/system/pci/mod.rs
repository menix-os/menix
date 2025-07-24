//! PCI bus implementation

pub mod device;
pub mod driver;

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

    // TODO: Enumerate buses.
}
