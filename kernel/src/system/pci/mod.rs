use alloc::vec::Vec;

use crate::system::pci::{
    config::{
        ACCESS, Access, Address,
        common::{DEVICE_ID, VENDOR_ID},
    },
    device::{DEVICES, Device},
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
                scan_device(addr, access.as_ref(), &mut devices);
            }
        }
    }
}

fn scan_device(addr: Address, access: &'static dyn Access, devices: &mut Vec<Device>) {
    let vendor_id = access.read16(addr.clone(), VENDOR_ID.offset() as _);
    if vendor_id == 0xFFFF {
        return;
    }

    let device_id = access.read16(addr.clone(), DEVICE_ID.offset() as _);
    if device_id == 0xFFFF {
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
