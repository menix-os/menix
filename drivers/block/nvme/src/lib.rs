#![no_std]

use menix::generic::cmdline::CmdLine;
use pci::{
    device::PciDevice,
    driver::{PciDriver, PciError, PciVariant},
};

menix::module!("NVMe block devices", "Marvin Friedrich", main);

static DRIVER: PciDriver = PciDriver {
    name: "nvme",
    probe: probe,
    remove: None,
    suspend: None,
    sleep: None,
    variants: &[PciVariant::new().class(1).sub_class(8)],
};

pub fn probe(_dev: &PciDevice) -> Result<(), PciError> {
    todo!();
}

pub fn main(_args: CmdLine) {
    _ = DRIVER.register();
}
