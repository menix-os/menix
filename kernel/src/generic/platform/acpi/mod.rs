// Advanced Configuration and Power Interface
// Wrapper for uACPI

mod uacpi;

use crate::{
    arch::platform,
    generic::{
        self,
        clock::{self},
        cpu::CpuData,
        memory::{
            self, PhysAddr,
            virt::{KERNEL_PAGE_TABLE, VmFlags},
        },
    },
};
use alloc::boxed::Box;
use core::{
    alloc::{Allocator, GlobalAlloc, Layout},
    ffi::{CStr, c_void},
    ptr::{NonNull, null_mut},
};
use spin::{Once, Spin, mutex::Mutex};
use uacpi::*;

static RSDP_ADDRESS: Once<PhysAddr> = Once::new();

pub fn init(rsdp: PhysAddr) {
    RSDP_ADDRESS.call_once(|| return rsdp);

    let mut uacpi_status = uacpi::uacpi_status_UACPI_STATUS_OK;

    // Get an early table window so we can initialize e.g. HPET and MADT.
    let mut early_mem = Box::<[u8]>::new_uninit_slice(4096);
    uacpi_status = unsafe {
        uacpi::uacpi_setup_early_table_access(
            early_mem.as_mut_ptr() as *mut c_void,
            early_mem.len(),
        )
    };
    if uacpi_status != uacpi_status_UACPI_STATUS_OK {
        error!(
            "acpi: Early table access failed with error {}!\n",
            uacpi_status
        );
        return;
    }

    #[cfg(target_arch = "x86_64")]
    clock::switch(Box::new(platform::Hpet::default()));

    print!("acpi: Initializing...\n");
    uacpi_status = unsafe { uacpi::uacpi_initialize(0) };
    if uacpi_status != uacpi_status_UACPI_STATUS_OK {
        error!(
            "acpi: Initialization failed with error \"{}\"!\n",
            uacpi_status
        );
    }

    // TODO: Evaluate MADT and initialize all remaining CPUs.
    // print!("acpi: Booting CPUs using MADT\n");

    uacpi_status = unsafe { uacpi::uacpi_namespace_load() };

    if uacpi_status != uacpi_status_UACPI_STATUS_OK {
        error!(
            "acpi: Namespace loading failed with error \"{}\"!\n",
            uacpi_status
        );
    } else {
        unsafe { uacpi::uacpi_namespace_initialize() };
    }
}
