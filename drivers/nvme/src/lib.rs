#![no_std]

use menix::system::pci::{
    PciError,
    device::PciDevice,
    driver::{PciDriver, PciVariant},
};

menix::module!("NVMe block devices", "Marvin Friedrich", main);

static DRIVER: PciDriver = PciDriver {
    name: "nvme",
    probe: probe,
    remove: None,
    suspend: None,
    sleep: None,
    variants: &[PciVariant::new().class(1).sub_class(8).function(2)],
};

pub fn probe(_dev: &PciDevice) -> Result<(), PciError> {
    todo!();
}

pub fn main() {
    _ = DRIVER.register();
}
