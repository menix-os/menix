use alloc::vec::Vec;

use crate::{
    generic::memory::view::MemoryView,
    system::pci::{
        config::{
            ACCESS, Access, Address, DeviceView,
            common::{DEVICE_ID, REG0, VENDOR_ID},
        },
        device::{DEVICES, Device},
    },
};

pub mod config;
pub mod device;
pub mod driver;

/// Initializes the PCI subsystem.
#[initgraph::task(
    name = "system.pci",
    entails = [crate::INIT_STAGE]
)]
#[cfg_attr(
    feature = "acpi",
    initgraph::task(depends = [super::acpi::INIT_STAGE])
)]
pub fn PCI_STAGE() {
    log!("Scanning PCI devices");

    let mut devices = DEVICES.lock();
    for access in ACCESS.get().iter() {
        for bus in access.start_bus()..=access.end_bus() {
            for slot in 0..32 {
                let addr = Address {
                    segment: access.segment(),
                    bus,
                    slot,
                    function: 0,
                };
                scan_device(addr, access.view_for_device(addr).unwrap(), &mut devices);
            }
        }
    }
}

fn scan_device<A: Access + ?Sized>(
    addr: Address,
    view: DeviceView<'_, A>,
    devices: &mut Vec<Device>,
) {
    let reg0 = view.read_reg(REG0).unwrap();

    let vendor_id = reg0.read_field(VENDOR_ID).value();
    if vendor_id == 0xFFFF || vendor_id == 0 {
        return;
    }

    let device_id = reg0.read_field(DEVICE_ID).value();
    if device_id == 0xFFFF || device_id == 0 {
        return;
    }

    log!(
        "{}: Vendor={:04x} Device={:04x}",
        addr,
        vendor_id,
        device_id
    );

    devices.push(Device {
        address: addr,
        driver: None,
    });
}
