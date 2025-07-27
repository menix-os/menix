// PCI driver handling.

use crate::generic::util::spin_mutex::SpinMutex;

use super::{PciError, device::PciDevice};
use alloc::collections::btree_map::BTreeMap;

pub type PciDriverFn = fn(dev: &PciDevice) -> Result<(), PciError>;

/// Represents a driver.
#[derive(Debug, Clone, Copy)]
pub struct PciDriver {
    /// The name of this driver.
    pub name: &'static str,
    /// Called when a new device is being connected.
    /// This function is mandatory.
    pub probe: PciDriverFn,
    ///  Called when a device is being removed.
    pub remove: Option<PciDriverFn>,
    ///  Called when a device is put to sleep.
    pub suspend: Option<PciDriverFn>,
    ///  Called when a device is woken up.
    pub sleep: Option<PciDriverFn>,
    /// Variants of devices that this driver can control.
    pub variants: &'static [PciVariant],
}

static DRIVERS: SpinMutex<BTreeMap<&'static str, PciDriver>> = SpinMutex::new(BTreeMap::new());

impl PciDriver {
    pub fn register(self) -> Result<(), PciError> {
        let mut drivers = DRIVERS.lock();

        if drivers.contains_key(self.name) {
            return Err(PciError::DriverAlreadyExists);
        }

        drivers.insert(self.name, self);

        log!(
            "Registered new PCI driver \"{}\" with {} variant{}",
            self.name,
            self.variants.len(),
            if self.variants.len() != 1 { "s" } else { "" }
        );

        return Ok(());
    }
}

/// Drivers can use this to create bindings.
/// Any field that is a [`Some`] variant will be matched on.
#[derive(Debug, Clone, Copy)]
pub struct PciVariant {
    /// Match the primary vendor ID.
    vendor: Option<u16>,
    /// Match the primary device ID.
    device: Option<u16>,
    /// Match the secondary vendor ID.
    sub_vendor: Option<u16>,
    /// Match the secondary device ID.
    sub_device: Option<u16>,
    /// Match a generic class.
    class: Option<u8>,
    /// Match a generic sub-class.
    sub_class: Option<u8>,
    /// Match a generic function.
    prog_if: Option<u8>,
    /// Driver-specific data.
    data: usize,
}

impl PciVariant {
    pub const fn new() -> Self {
        Self {
            vendor: None,
            device: None,
            sub_vendor: None,
            sub_device: None,
            class: None,
            sub_class: None,
            prog_if: None,
            data: 0,
        }
    }

    pub const fn id(mut self, vendor: u16, device: u16) -> Self {
        self.vendor = Some(vendor);
        self.device = Some(device);
        return self;
    }

    pub const fn sub_id(mut self, sub_vendor: u16, sub_device: u16) -> Self {
        self.sub_vendor = Some(sub_vendor);
        self.sub_device = Some(sub_device);
        return self;
    }

    pub const fn class(mut self, class: u8) -> Self {
        self.class = Some(class);
        return self;
    }

    pub const fn sub_class(mut self, sub_class: u8) -> Self {
        self.sub_class = Some(sub_class);
        return self;
    }

    pub const fn function(mut self, prog_if: u8) -> Self {
        self.prog_if = Some(prog_if);
        return self;
    }

    pub const fn with_data(mut self, data: usize) -> Self {
        self.data = data;
        return self;
    }
}
