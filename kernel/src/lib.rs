#![no_std]
#![feature(negative_impls)]
#![feature(allocator_api)]
#![feature(str_from_raw_parts)]
#![feature(new_zeroed_alloc)]
#![feature(likely_unlikely)]
#![feature(slice_split_once)]
#![feature(bool_to_result)]
#![feature(box_into_inner)]
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
    process::{Identity, Process},
    util::{mutex::irq::IrqMutex, once::Once},
    vfs::{File, file::OpenFlags, inode::Mode},
};
use alloc::{string::String, sync::Arc};
use core::hint;
use generic::boot::BootInfo;

/// Initializes all important kernel structures.
/// This is invoked by the boot environment.
pub fn init() -> ! {
    {
        let _irq = IrqMutex::lock();
        generic::init::run();
    }

    loop {
        hint::spin_loop();
    }
}

static INIT: Once<Arc<Process>> = Once::new();

/// The high-level kernel entry point.
pub extern "C" fn main(_: usize, _: usize) {
    // Say hello to the console.
    log!("Menix {}", env!("CARGO_PKG_VERSION"));

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

    // Open /dev/console for stdio for init.
    {
        let mut init_inner = init_proc.inner.lock();
        let console = File::open(
            &init_inner,
            None,
            b"/dev/console",
            OpenFlags::ReadWrite,
            Mode::empty(),
            Identity::get_kernel(),
        )
        .expect("Unable to open console for init");

        init_inner.open_files.insert(0, console.clone());
        init_inner.open_files.insert(1, console.clone());
        init_inner.open_files.insert(2, console);
    }

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
}
