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

use crate::generic::{process::Process, vfs::path::PathBuf};
use generic::{boot::BootInfo, memory::virt, percpu::CpuData, process::task::Task};

// TODO: Instead of having global init functions, use an initgraph with distinguishable stages.
unsafe fn run_init_tasks(start: *const fn(), end: *const fn()) {
    let mut cur = start;
    while cur < end {
        unsafe {
            (*cur)();
            cur = cur.add(1);
        }
    }
}

/// Initializes all important kernel structures.
/// This is invoked by the prekernel environment.
pub fn main() -> ! {
    unsafe {
        arch::core::setup_bsp();
        generic::memory::init();
    }

    // Run early init calls.
    unsafe {
        run_init_tasks(
            &raw const virt::LD_EARLY_ARRAY_START as *const fn(),
            &raw const virt::LD_EARLY_ARRAY_END as *const fn(),
        );
    }

    // Say hello to the console.
    log!(
        "{} {} {} {}",
        generic::posix::utsname::SYSNAME,
        generic::posix::utsname::RELEASE,
        generic::posix::utsname::VERSION,
        generic::posix::utsname::MACHINE
    );

    log!("Command line: {}", BootInfo::get().command_line.inner());

    generic::vfs::init();
    generic::module::init();
    system::init();

    // Run init calls.
    unsafe {
        run_init_tasks(
            &raw const virt::LD_INIT_ARRAY_START as *const fn(),
            &raw const virt::LD_INIT_ARRAY_END as *const fn(),
        );
    }

    // Set up scheduler.
    let init = Task::new(run_init, 0, 0, None, false).expect("Couldn't create kernel task");
    CpuData::get().scheduler.start(init);
}

/// The high-level kernel entry point. This is invoked by the scheduler once it's running.
extern "C" fn run_init(_: usize, _: usize) {
    // Find init. If no path is given, search a few select directories.
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
