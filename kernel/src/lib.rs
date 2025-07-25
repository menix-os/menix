#![no_std]
#![feature(negative_impls)]
#![feature(allocator_api)]
#![feature(str_from_raw_parts)]
#![feature(new_zeroed_alloc)]
#![feature(likely_unlikely)]
#![feature(slice_split_once)]
#![no_builtins]
// Clippy lints
#![allow(clippy::needless_return)]
#![allow(clippy::new_without_default)]
#![forbid(clippy::missing_safety_doc)]
#![forbid(clippy::large_stack_frames)]

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
    process::{Identity, Process, task::Task},
    util::once::Once,
    vfs::{File, file::OpenFlags, inode::Mode},
};
use alloc::{string::String, sync::Arc};
use core::hint;
use generic::boot::BootInfo;

/// Initializes all important kernel structures.
/// This is invoked by the prekernel environment.
pub fn init() -> ! {
    unsafe {
        arch::irq::set_irq_state(false);
        generic::init::run();
        arch::irq::set_irq_state(true);
    }

    CpuData::get().scheduler.add_task(Arc::new(
        Task::new(crate::main, 0, 0, Process::get_kernel(), false)
            .expect("Couldn't create kernel task"),
    ));

    loop {
        hint::spin_loop();
    }
}

static INIT: Once<Arc<Process>> = Once::new();

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

    unsafe {
        INIT.init(Arc::new(
            Process::new("init".into(), None).expect("Unable to create init process"),
        ))
    };

    let init_proc = INIT.get();
    let init_file = {
        let init_inner = init_proc.inner.lock();
        File::open(
            &init_inner,
            None,
            path,
            OpenFlags::ReadOnly | OpenFlags::Executable,
            Mode::empty(),
            &Identity::get_kernel(),
        )
        .expect("Unable to read the init executable")
    };

    init_proc
        .clone()
        .fexecve(init_file, &[path], &[])
        .expect("Unable to start the init process");

    loop {
        unsafe { arch::irq::set_irq_state(true) };
    }
}
