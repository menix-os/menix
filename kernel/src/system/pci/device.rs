use crate::{
    generic::util::mutex::spin::SpinMutex,
    system::pci::{config::Address, driver::Driver},
};
use alloc::vec::Vec;

pub struct Device {
    pub address: Address,
    /// The driver currently bound to this device.
    pub driver: Option<&'static Driver>,
}

pub static DEVICES: SpinMutex<Vec<Device>> = SpinMutex::new(Vec::new());
