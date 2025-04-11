#![no_std]

use menix::generic::cmdline::CmdLine;

pub mod device;
pub mod driver;

menix::module!("PCI/PCIe bus subsystem", "Marvin Friedrich", main);

pub fn main(_args: CmdLine) {
    // TODO: Get PCI addresses via boot info.
    // TODO: Scan for PCI devices.
}
