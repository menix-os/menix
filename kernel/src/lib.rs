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

unsafe extern "C" {
    unsafe static LD_EARLY_ARRAY_START: u8;
    unsafe static LD_EARLY_ARRAY_END: u8;
    unsafe static LD_INIT_ARRAY_START: u8;
    unsafe static LD_INIT_ARRAY_END: u8;
}

/// The high-level kernel entry point. This is invoked by the prekernel environment.
#[unsafe(no_mangle)]
pub(crate) fn main() -> ! {
    arch::core::setup_bsp();

    // Run early init calls.
    unsafe {
        let mut early_array = &raw const LD_EARLY_ARRAY_START as *const fn();
        let early_end = &raw const LD_EARLY_ARRAY_END as *const fn();
        while early_array < early_end {
            (*early_array)();
            early_array = early_array.add(1);
        }
    }

    // Initialize memory management.
    unsafe { generic::memory::init() };

    arch::core::perpare_cpu(unsafe { arch::core::get_per_cpu().as_mut().unwrap() });
    // TODO: Initialize VFS.

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
    generic::platform::init();

    // Run init calls.
    unsafe {
        let mut init_array = &raw const LD_INIT_ARRAY_START as *const fn();
        let init_end = &raw const LD_INIT_ARRAY_END as *const fn();
        while init_array < init_end {
            (*init_array)();
            init_array = init_array.add(1);
        }
    }

    // Load all modules and run their init function.
    //generic::module::init();

    // Say hello to the console.
    // TODO: Get this information from posix/utsname instead.
    log!(
        "Menix {}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );

    // TODO: Setup SMP.

    // TODO: Start scheduler.

    log!("boot: Starting init...");
    todo!("Load init");
}
