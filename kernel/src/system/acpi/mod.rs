mod uacpi;

use crate::generic::{boot::BootInfo, memory::PhysAddr, util::once::Once};
use alloc::boxed::Box;
use core::ffi::c_void;

static RSDP_ADDRESS: Once<PhysAddr> = Once::new();

#[initgraph::task(
    name = "system.acpi.tables",
    depends = [crate::generic::memory::MEMORY_STAGE],
)]
pub fn TABLES_STAGE() {
    match BootInfo::get().rsdp_addr {
        Some(rsdp) => unsafe { RSDP_ADDRESS.init(rsdp) },
        None => panic!("No RSDP available, unable to initialize the ACPI subsystem!"),
    };

    // Get an early table window so we can initialize e.g. HPET and MADT.
    let mut early_mem = Box::<[u8]>::new_uninit_slice(4096);

    let uacpi_status = unsafe {
        uacpi::uacpi_setup_early_table_access(
            early_mem.as_mut_ptr() as *mut c_void,
            early_mem.len(),
        )
    };

    if uacpi_status != uacpi::UACPI_STATUS_OK {
        error!(
            "acpi: Early table access failed with error {}!\n",
            uacpi_status
        );
        return;
    }
}

#[initgraph::task(
    name = "system.acpi",
    depends = [
        TABLES_STAGE,
        crate::arch::INIT_STAGE,
        crate::generic::clock::CLOCK_STAGE,
        crate::generic::memory::MEMORY_STAGE,
    ],
)]
pub fn INIT_STAGE() {
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
