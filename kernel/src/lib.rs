#![no_std]
#![feature(negative_impls)]
#![feature(allocator_api)]
#![feature(str_from_raw_parts)]
#![feature(likely_unlikely)]
#![feature(slice_split_once)]
#![feature(bool_to_result)]
#![feature(box_into_inner)]
#![feature(slice_as_array)]
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
pub mod boot;
pub mod clock;
pub mod cmdline;
pub mod device;
pub mod fbcon;
pub mod init;
pub mod irq;
pub mod log;
pub mod memory;
pub mod module;
pub mod panic;
pub mod percpu;
pub mod posix;
pub mod process;
pub mod sched;
pub mod syscall;
pub mod util;
pub mod vfs;

pub mod system;

use core::sync::atomic::AtomicBool;

use crate::{
    percpu::CpuData,
    process::{Identity, Process},
    util::{mutex::irq::IrqMutex, once::Once},
    vfs::{
        File,
        file::{FileDescription, OpenFlags},
        fs::initramfs::load,
        inode::Mode,
    },
};
use alloc::{string::String, sync::Arc};
use boot::BootInfo;

/// Initializes all important kernel structures.
/// This is invoked by the boot environment.
pub fn init() -> ! {
    {
        let _irq = IrqMutex::lock();
        init::run();
    }

    CpuData::get().scheduler.do_yield();
    unreachable!("The scheduler got back to menix::init?");
}

static INIT: Once<Arc<Process>> = Once::new();

/// The high-level kernel entry point.
pub extern "C" fn main(_: usize, _: usize) {
    // Say hello to the console.
    log!("Menix {}", env!("CARGO_PKG_VERSION"));
    log!("Command line: {}", BootInfo::get().command_line.inner());

    // Load the initramfs.
    {
        let proc_inner = Process::get_kernel().inner.lock();
        // Load the initramfs into the root directory.
        let root_dir = File::open(
            &proc_inner,
            None,
            b"/",
            OpenFlags::Directory,
            Mode::empty(),
            Identity::get_kernel(),
        )
        .expect("Unable to open root directory");

        for file in BootInfo::get().files {
            load(&proc_inner, root_dir.clone(), unsafe {
                core::slice::from_raw_parts(file.data.as_hhdm(), file.length)
            })
            .expect("Failed to load one of the provided initramfs archives");
        }
    }

    // Find user space init. If no path is given, search a few select directories.
    let path = match BootInfo::get().command_line.get_string("init") {
        Some(x) => x.as_bytes(),
        // TODO: Search VFS for alternative init paths like "/sbin/init"
        None => b"/init",
    };

    log!("Starting init \"{}\"", String::from_utf8_lossy(path));

    let args = vec![path.to_vec()];
    log!("With arguments:");
    args.iter()
        .for_each(|x| log!("    {}", String::from_utf8_lossy(x)));

    let envs = vec![b"HOME=/".to_vec(), b"TERM=xterm-256color".to_vec()];
    log!("With environment:");
    envs.iter()
        .for_each(|x| log!("    {}", String::from_utf8_lossy(x)));

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

        for i in 0..=2 {
            init_inner.open_files.insert(
                i,
                FileDescription {
                    file: console.clone(),
                    close_on_exec: AtomicBool::new(false),
                },
            );
        }
    }

    let init_file = {
        let init_inner = init_proc.inner.lock();
        File::open(
            &init_inner,
            None,
            path,
            OpenFlags::Read | OpenFlags::Executable,
            Mode::empty(),
            Identity::get_kernel(),
        )
        .expect("Unable to read the init executable")
    };

    init_proc
        .clone()
        .fexecve(init_file, args, envs)
        .expect("Unable to start the init process");
}

/// This stage should be used to express that the system must be initialized at this point.
#[initgraph::task(name = "system")]
pub fn INIT_STAGE() {}
