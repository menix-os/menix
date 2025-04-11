// PCI driver handling.

use crate::device::PciDevice;
use menix::{alloc::collections::btree_map::BTreeMap, spin::RwLock};

#[derive(Debug)]
pub enum PciError {
    Unknown,
    DriverAlreadyExists,
}

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

static DRIVERS: RwLock<BTreeMap<&'static str, PciDriver>> = RwLock::new(BTreeMap::new());

impl PciDriver {
    pub fn register(self) -> Result<(), PciError> {
        let mut drivers = DRIVERS.write();

        if drivers.contains_key(self.name) {
            return Err(PciError::DriverAlreadyExists);
        }

        drivers.insert(self.name, self);

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

    pub const fn id(self, vendor: u16, device: u16) -> Self {
        let mut result = self;
        result.vendor = Some(vendor);
        result.device = Some(device);
        return result;
    }

    pub const fn sub_id(self, sub_vendor: u16, sub_device: u16) -> Self {
        let mut result = self;
        result.sub_vendor = Some(sub_vendor);
        result.sub_device = Some(sub_device);
        return result;
    }

    pub const fn class(self, class: u8) -> Self {
        let mut result = self;
        result.class = Some(class);
        return result;
    }

    pub const fn sub_class(self, sub_class: u8) -> Self {
        let mut result = self;
        result.sub_class = Some(sub_class);
        return result;
    }

    pub const fn function(self, prog_if: u8) -> Self {
        let mut result = self;
        result.prog_if = Some(prog_if);
        return result;
    }

    pub const fn with_data(self, data: usize) -> Self {
        let mut result = self;
        result.data = data;
        return result;
    }
}
