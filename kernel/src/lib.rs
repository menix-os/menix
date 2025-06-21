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

use crate::generic::{
    percpu::CpuData,
    process::{Process, sched::Scheduler, task::Task},
};
use alloc::{string::String, sync::Arc};
use core::hint;
use generic::boot::BootInfo;

/// Initializes all important kernel structures.
/// This is invoked by the prekernel environment.
pub fn init() -> ! {
    generic::init::run();

    // Set up scheduler.
    let bsp_scheduler = &CpuData::get().scheduler;
    bsp_scheduler.add_task(Arc::new(
        Task::new(main, 0, 0, Process::get_kernel(), false).expect("Couldn't create kernel task"),
    ));
    bsp_scheduler.prepare();

    loop {
        hint::spin_loop();
    }
}

/// The high-level kernel entry point.
pub extern "C" fn main(_: usize, _: usize) {
    // Say hello to the console.
    log!(
        "{} {} {} {}",
        generic::posix::utsname::SYSNAME,
        generic::posix::utsname::RELEASE,
        generic::posix::utsname::VERSION,
        generic::posix::utsname::MACHINE
    );

    log!("Command line: {}", BootInfo::get().command_line.inner());

    // Find user space init. If no path is given, search a few select directories.
    let path = match BootInfo::get().command_line.get_string("init") {
        Some(x) => x.as_bytes(),
        // TODO: Search VFS for alternative init paths like "/sbin/init"
        None => b"/init",
    };

    log!("Starting init \"{}\"", String::from_utf8_lossy(path));

    let kernel_proc = Scheduler::get_current().get_process();
    let init_proc = Process::from_file(None, path).expect("Unable to create init process");
    // TODO: Add to run queue.

    loop {
        // TODO: For some reason going past this triggers a #UD.
    }
}
