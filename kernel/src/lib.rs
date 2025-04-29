#![no_std]
#![allow(unused)]
#![allow(clippy::needless_return)]
#![feature(negative_impls)]
#![feature(allocator_api)]
// Needed for volatile memmove. This is an LLVM intrinsic, replace it with our own.
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(str_from_raw_parts)]
#![feature(new_zeroed_alloc)]
#![feature(cfg_match)]
#![feature(likely_unlikely)]
#![no_builtins]

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
#[unsafe(no_mangle)]
pub(crate) fn main() -> ! {
    generic::cpu::setup_bsp();
    generic::memory::init::init();

    // TODO: Run early init calls.

    // Initialize allocators.
    generic::memory::init();

    // TODO: Run init calls.

    // Initialize early console.
    // TODO: Abstract console interface so it handles initialization of all consoles as well.
    if BootInfo::get()
        .command_line
        .get_bool("fbcon")
        .unwrap_or(true)
    {
        if let Some(fb) = &BootInfo::get().framebuffer {
            generic::boot::fbcon::init(fb.clone());
        }
    }

    // Say hello to the console.
    // TODO: Get this information from posix/utsname instead.
    print!(
        "Menix {}.{}.{}\n",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );

    // Load the platform subsystems.
    generic::platform::init();

    // Setup SMP.
    generic::cpu::setup_all();

    // TODO: Start scheduler.

    // Initialize buses.
    generic::bus::init();

    // Load all modules and run their init function.
    generic::module::init();

    print!("boot: Starting init...\n");
    todo!("Load init");
}
