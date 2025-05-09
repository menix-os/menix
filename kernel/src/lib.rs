#![no_std]
#![feature(negative_impls)]
#![feature(allocator_api)]
#![feature(str_from_raw_parts)]
#![feature(new_zeroed_alloc)]
#![feature(cfg_match)]
#![feature(likely_unlikely)]
#![no_builtins]
// Clippy lints
#![allow(clippy::needless_return)]
#![allow(clippy::new_without_default)]
#![forbid(clippy::missing_safety_doc)]

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

/// Runs early init functions. This has to be called as the very first thing after `_start`!
/// # Safety
/// Not calling this function will lead to undefined behavior!
#[forbid(dead_code)]
pub(crate) unsafe fn early_init() {
    arch::core::setup_bsp();

    // Run early init calls. These don't need memory allocations.
    unsafe {
        let mut early_array = &raw const LD_EARLY_ARRAY_START as *const fn();
        let early_end = &raw const LD_EARLY_ARRAY_END as *const fn();
        while early_array < early_end {
            (*early_array)();
            early_array = early_array.add(1);
        }
    }
}

#[forbid(dead_code)]
pub(crate) fn init() {
    unsafe {
        let mut init_array = &raw const LD_INIT_ARRAY_START as *const fn();
        let init_end = &raw const LD_INIT_ARRAY_END as *const fn();
        while init_array < init_end {
            (*init_array)();
            init_array = init_array.add(1);
        }
    }
}

/// The high-level kernel entry point. This is invoked by the prekernel environment.
#[unsafe(no_mangle)]
pub(crate) fn main() -> ! {
    // Say hello to the console.
    // TODO: Get this information from posix/utsname instead.
    log!(
        "Menix {}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );

    // Initialize memory management.
    unsafe { generic::memory::init() };

    // TODO: Initialize virtual file system.
    // generic::posix::fs::init();

    generic::platform::init();

    arch::core::perpare_cpu(generic::percpu::CpuData::get());

    // Run init calls.
    init();

    // Load all modules and run their init function.
    generic::module::init();

    // TODO: Setup SMP.

    // Find init. If no path is given, search a few select directories.
    let path = match generic::boot::BootInfo::get()
        .command_line
        .get_string("init")
    {
        Some(x) => x,
        // TODO: Search filesystem for init binaries.
        None => "/usr/sbin/init",
    };
    log!("Starting init \"{}\"", path);

    // TODO: Start init.
    loop {
        core::hint::spin_loop();
    }
}
