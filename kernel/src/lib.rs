#![no_std]
#![allow(unused)]
#![allow(clippy::needless_return)]
#![feature(negative_impls)]
#![feature(naked_functions)]
#![feature(allocator_api)]
// Needed for volatile memmove
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(str_from_raw_parts)]
#![feature(new_zeroed_alloc)]
#![feature(cfg_match)]

use generic::boot::BootInfo;

pub extern crate alloc;
pub extern crate core;

#[macro_use]
pub mod macros;
pub mod arch;
pub mod generic;

/// The high-level kernel entry point.
/// Called by `_start`.
/// Initializes all subsystems and starts all servers.
#[deny(dead_code)]
pub(crate) fn main() -> ! {
    #[cfg(feature = "boot_limine")]
    generic::boot::limine::init();

    let info = BootInfo::get();

    let hhdm = info
        .hhdm_address
        .expect("HHDM address should have been set!");

    generic::memory::init(&info.memory_map, hhdm);

    generic::memory::virt::init(hhdm, info.kernel_phys, info.kernel_virt);

    // Initialize early console.
    // TODO: Abstract console interface so it handles init as well.
    if let Some(fb) = &info.framebuffer {
        generic::boot::bootcon::init(fb.clone());
    }

    generic::cpu::setup_bsp();

    // TODO: Get this information from posix/utsname instead.
    print!(
        "Menix {}.{}.{}\n",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );

    // Load the ACPI subsystem.
    #[cfg(feature = "acpi")]
    if let Some(rsdp) = info.rsdp_addr {
        generic::platform::acpi::init(rsdp);
    }

    // Initialize buses.
    generic::bus::init();

    // Finally, load all modules.
    generic::module::init();

    print!("boot: Starting init...\n");
    todo!("Load init");
}
