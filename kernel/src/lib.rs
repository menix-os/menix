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
    let info = BootInfo::get();

    unsafe { arch::irq::interrupt_disable() };
    generic::cpu::setup_bsp();

    // Initialize allocators.
    {
        let hhdm = info
            .hhdm_address
            .expect("HHDM address should have been set!");
        generic::memory::init(&info.memory_map, hhdm);
        generic::memory::virt::init(hhdm, info.kernel_phys, info.kernel_virt);
    }

    // Say hello to the console.
    // TODO: Get this information from posix/utsname instead.
    print!(
        "Menix {}.{}.{}\n",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );

    // Initialize early console.
    // TODO: Abstract console interface so it handles initialization of all consoles as well.
    if let Some(fb) = &info.framebuffer {
        generic::boot::bootcon::init(fb.clone());
    }

    // Load the ACPI subsystem.
    #[cfg(feature = "acpi")]
    if let Some(rsdp) = info.rsdp_addr {
        generic::platform::acpi::init(rsdp);
    }

    unsafe { arch::irq::interrupt_enable() };

    // TODO: Start scheduler.

    // Setup SMP.
    generic::cpu::setup_all();

    // Initialize buses.
    generic::bus::init();

    // Load all modules and run their init function.
    generic::module::init();

    print!("boot: Starting init...\n");
    todo!("Load init");
}
