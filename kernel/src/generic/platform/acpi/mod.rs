mod uacpi;

use crate::generic::{boot::BootInfo, memory::PhysAddr};
use alloc::boxed::Box;
use core::ffi::c_void;
use spin::Once;

static RSDP_ADDRESS: Once<PhysAddr> = Once::new();

pub fn init() {
    match BootInfo::get().rsdp_addr {
        Some(rsdp) => RSDP_ADDRESS.call_once(|| rsdp),
        None => panic!("No RSDP available, unable to initialize the ACPI subsystem!"),
    };

    let mut uacpi_status = unsafe { uacpi::uacpi_initialize(0) };
    if uacpi_status != uacpi::UACPI_STATUS_OK {
        error!(
            "acpi: Initialization failed with error \"{}\"!",
            uacpi_status
        );
        return;
    }

    uacpi_status = unsafe { uacpi::uacpi_namespace_load() };
    if uacpi_status != uacpi::UACPI_STATUS_OK {
        error!(
            "acpi: Namespace loading failed with error \"{}\"!",
            uacpi_status
        );
    } else {
        unsafe { uacpi::uacpi_namespace_initialize() };
    }
}
