//! PCI bus implementation

pub mod device;
pub mod driver;

#[derive(Debug)]
pub enum PciError {
    Unknown,
    DriverAlreadyExists,
}

init_stage! {
    pub PCI_STAGE: "system.pci" => init;
}

/// Initializes the PCI subsystem.
fn init() {
    log!("Initializing the PCI subsystem");

    // TODO: Enumerate buses.
}
