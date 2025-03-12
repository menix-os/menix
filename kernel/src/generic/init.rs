// Kernel initialization.

use crate::{
    arch::{self, PhysAddr, VirtAddr},
    boot::BootInfo,
    generic::{
        self, elf,
        log::{KernelLogger, Logger},
        phys::{PhysManager, PhysMemory},
        thread::Thread,
        virt::{self, PageTable},
    },
};
use alloc::{boxed::Box, sync::Arc};
use spin::RwLock;

// The boot process is split into 3 stages.
// - `early_init`: Very early calls that don't need dynamic memory allocations.
// - `memory_init`: Calls to evaluate the memory map and setup allocators.
// - `init`: Calls to initialize the rest of the kernel.

/// Called as the very first thing during boot. Initializes very basic I/O and temporary features.
pub(crate) fn early_init() {
    arch::init::early_init();
}

/// Called as soon as a memory map is available.
/// All code ran after this stage can use dynamic allocations.
pub(crate) fn memory_init(
    memory_map: &mut [PhysMemory],
    identity_base: VirtAddr,
    kernel_phys: PhysAddr,
    kernel_virt: VirtAddr,
) {
    PhysManager::init(memory_map, identity_base);

    // From now on, we can save logs in memory.
    // TODO: Deadlocks
    // Logger::add_sink(Box::new(KernelLogger));

    print!("boot: Memory map provided by bootloder:\n");
    print!("{:^16} {:^16} {}\n", "Address", "Length", "Usage");
    memory_map
        .iter()
        .for_each(|x| print!("{:>16x} {:>16x} {:?}\n", x.address, x.length, x.usage));

    virt::init(kernel_phys, kernel_virt);
}

/// Called after all info from the bootloader has been collected.
/// Initializes all subsystems and starts all servers.
pub(crate) fn init(info: &mut BootInfo) {
    print!(
        "Menix v{}\n",
        env!("CARGO_PKG_VERSION", "0 (Not built with cargo)")
    );

    match info.command_line {
        Some(x) => print!("boot: Command line: \"{x}\"\n"),
        None => print!("boot: Command line is empty.\n"),
    }

    generic::firmware::init(info);
    arch::init::init(info);

    // Load all modules.
    if let Some(files) = info.files {
        for file in files {
            print!("boot: Loading module \"{}\"\n", file.path);
            // TODO: Load the modules :^)
        }
    }

    print!("boot: Starting init\n");

    // Load init.
    // TODO:

    // let init = Process::from_elf(init_path);
}
