#![no_std]
#![feature(negative_impls)]
#![feature(allocator_api)]
#![feature(str_from_raw_parts)]
#![feature(new_zeroed_alloc)]
#![feature(likely_unlikely)]
#![no_builtins]
// Clippy lints
#![allow(clippy::needless_return)]
#![allow(clippy::new_without_default)]
#![forbid(clippy::missing_safety_doc)]

#[macro_use]
pub extern crate alloc;
#[macro_use]
pub extern crate core;

#[macro_use]
pub mod macros;
pub mod arch;
pub mod generic;
pub mod system;

use core::hint;

use crate::generic::{
    process::{Process, sched::Scheduler, task::Task},
    vfs::path::PathBuf,
};
use generic::boot::BootInfo;

/// Initializes all important kernel structures.
/// This is invoked by the prekernel environment.
pub fn init() -> ! {
    crate::generic::init::run();

    // Say hello to the console.
    log!(
        "{} {} {} {}",
        generic::posix::utsname::SYSNAME,
        generic::posix::utsname::RELEASE,
        generic::posix::utsname::VERSION,
        generic::posix::utsname::MACHINE
    );

    log!("Command line: {}", BootInfo::get().command_line.inner());

    // Set up scheduler.
    let init =
        Task::new(main, 0, 0, Process::get_kernel(), false).expect("Couldn't create kernel task");
    Scheduler::add_task(init);

    loop {
        hint::spin_loop();
    }
}

/// The high-level kernel entry point.
pub extern "C" fn main(_: usize, _: usize) {
    // Find user-space init. If no path is given, search a few select directories.
    let path = match BootInfo::get().command_line.get_string("init") {
        Some(x) => x,
        // TODO: Search filesystem for init binaries.
        None => "/sbin/init",
    };

    let path = PathBuf::from_str(path);

    log!("Starting init \"{}\"", path);
    let init = Process::from_file(&path).unwrap();
    // TODO: Add to run queue.
}
