mod uacpi;

use crate::generic::{boot::BootInfo, memory::PhysAddr, util::once::Once};
use alloc::boxed::Box;
use core::ffi::c_void;

static RSDP_ADDRESS: Once<PhysAddr> = Once::new();

init_stage! {
    #[depends(crate::arch::ARCH_STAGE, crate::generic::memory::MEMORY_STAGE)]
    pub TABLES_STAGE: "system.acpi.tables" => early_init;

    #[depends(TABLES_STAGE, crate::arch::ARCH_STAGE, crate::generic::clock::CLOCK_STAGE, crate::generic::memory::MEMORY_STAGE)]
    #[entails(super::pci::PCI_STAGE)]
    pub INIT_STAGE: "system.acpi" => init;
}

fn early_init() {
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

fn init() {
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
