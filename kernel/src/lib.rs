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

use generic::{percpu::CpuData, sched::task::Task};

/// Initializes all important kernel structures.
/// This is invoked by the prekernel environment.
pub(crate) fn main() -> ! {
    unsafe {
        arch::core::setup_bsp();
        generic::memory::init();
    }

    // Run early init calls.
    unsafe {
        let mut early_array =
            &raw const generic::memory::virt::LD_EARLY_ARRAY_START as *const fn();
        let early_end = &raw const generic::memory::virt::LD_EARLY_ARRAY_END as *const fn();
        while early_array < early_end {
            (*early_array)();
            early_array = early_array.add(1);
        }
    }

    // Say hello to the console.
    log!(
        "{} {} {} {}",
        generic::posix::utsname::SYSNAME,
        generic::posix::utsname::RELEASE,
        generic::posix::utsname::VERSION,
        generic::posix::utsname::MACHINE
    );

    generic::posix::fs::init();
    generic::platform::init();

    // Run init calls.
    unsafe {
        let mut init_array = &raw const generic::memory::virt::LD_INIT_ARRAY_START as *const fn();
        let init_end = &raw const generic::memory::virt::LD_INIT_ARRAY_END as *const fn();
        while init_array < init_end {
            (*init_array)();
            init_array = init_array.add(1);
        }
    }

    generic::module::init();

    // Set up scheduler.
    let init = Task::new(run_init, 0, None, false).expect("Couldn't create kernel task");
    CpuData::get().scheduler.start(init);
}

/// The high-level kernel entry point. This is invoked by the scheduler once it's running.
extern "C" fn run_init(_: usize) -> ! {
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
    unreachable!();
}
