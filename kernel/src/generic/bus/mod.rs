#[cfg(feature = "pci")]
pub mod pci;

/// Initializes all buses.
#[deny(dead_code)]
pub(crate) fn init() {
    print!("bus: Initializing all buses.\n");

    #[cfg(feature = "pci")]
    pci::init();
}
