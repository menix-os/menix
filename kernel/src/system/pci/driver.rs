use super::device::Device;
use crate::{
    generic::{
        memory::view::MemoryView,
        posix::errno::{EResult, Errno},
        util::mutex::spin::SpinMutex,
    },
    system::pci::{
        config::{self, ACCESS, Access, DeviceView},
        device::DEVICES,
    },
};
use alloc::collections::btree_map::BTreeMap;

kernel_proc::pci_variant_builders! {
    MassStorageController = 0x01 {
        SerialAtaController = 0x06 {},
        NonVolatileMemoryController = 0x08 {
            NvmExpressController = 0x02,
        },
    },
    SerialBusController = 0x0C {
        UsbController = 0x03 {
            XhciController = 0x30,
        },
    },
}

/// Drivers can use this to create bindings.
/// Any field that is a [`Some`] variant will be matched on.
#[derive(Debug, Clone, Copy)]
pub struct PciVariant {
    /// Match the primary vendor ID.
    vendor: Option<u16>,
    /// Match the primary device ID.
    device: Option<u16>,
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
            class: None,
            sub_class: None,
            prog_if: None,
            data: 0,
        }
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

/// Represents a driver.
#[derive(Debug, Clone, Copy)]
pub struct Driver {
    /// The name of this driver.
    pub name: &'static str,
    /// Called when a new device is being connected.
    /// This function is mandatory.
    pub probe: fn(dev: &Device, access: DeviceView<'static, dyn Access>) -> EResult<()>,
    /// Called when a device is being removed.
    pub remove: Option<fn(dev: &Device) -> EResult<()>>,
    /// Called when a device is put to sleep.
    pub suspend: Option<fn(dev: &Device) -> EResult<()>>,
    /// Called when a device is woken up.
    pub resume: Option<fn(dev: &Device) -> EResult<()>>,
    /// Variants of devices that this driver can control.
    pub variants: &'static [PciVariant],
}

static DRIVERS: SpinMutex<BTreeMap<&'static str, &'static Driver>> =
    SpinMutex::new(BTreeMap::new());

impl Driver {
    pub fn register(&'static self) -> EResult<()> {
        let mut drivers = DRIVERS.lock();

        if drivers.contains_key(self.name) {
            return Err(Errno::EEXIST);
        }

        drivers.insert(self.name, self);

        log!(
            "Registered new PCI driver \"{}\" with {} variant(s)",
            self.name,
            self.variants.len()
        );

        // Probe matching PCI devices.
        let devices = DEVICES.lock();
        for dev in devices.iter() {
            let view = ACCESS
                .get()
                .iter()
                .filter_map(|x| x.view_for_device(dev.address))
                .next()
                .unwrap();

            let reg0 = view.read_reg(config::common::REG0).unwrap();
            let reg2 = view.read_reg(config::common::REG2).unwrap();

            let device_id = reg0.read_field(config::common::DEVICE_ID).value();
            let vendor_id = reg0.read_field(config::common::VENDOR_ID).value();
            let prog_if = reg2.read_field(config::common::PROG_IF).value();
            let sub_class = reg2.read_field(config::common::SUB_CLASS).value();
            let class = reg2.read_field(config::common::CLASS_CODE).value();

            if let Some(_) = self.variants.iter().find(|v| {
                v.device.is_none_or(|x| x == device_id)
                    && v.vendor.is_none_or(|x| x == vendor_id)
                    && v.prog_if.is_none_or(|x| x == prog_if)
                    && v.sub_class.is_none_or(|x| x == sub_class)
                    && v.class.is_none_or(|x| x == class)
            }) {
                if let Err(err) = (self.probe)(&dev, view) {
                    error!(
                        "Driver \"{}\" failed to probe device {}: {:?}",
                        self.name, dev.address, err
                    );
                }
            }
        }

        Ok(())
    }
}
