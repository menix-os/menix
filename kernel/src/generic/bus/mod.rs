#[cfg(feature = "pci")]
pub mod pci;

/// Initializes all buses.
pub(crate) fn init() {
    print!("bus: Initializing all buses.\n");

    #[cfg(feature = "pci")]
    pci::init();
}
