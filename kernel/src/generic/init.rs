// Kernel initialization.

use crate::{
    arch::{self, PhysAddr, VirtAddr},
    boot::BootInfo,
    firmware,
    generic::{
        elf,
        phys::{PhysManager, PhysMemory},
        task::Task,
        virt::{self, PageTable},
    },
};
use alloc::sync::Arc;
use spin::RwLock;

// The boot process is split into 3 stages.
// - `early_init`: Very early calls that don't need dynamic memory allocations.
// - `memory_init`: Calls to evaluate the memory map and setup allocators.
// - `init`: Calls to initialize the rest of the kernel.

/// Called as the very first thing during boot. Initializes very basic I/O and temporary features.
pub fn early_init() {
    arch::init::early_init();
}

/// Called as soon as a memory map is available.
/// All code ran after this stage can use dynamic allocations.
pub fn memory_init(
    memory_map: &mut [PhysMemory],
    identity_base: VirtAddr,
    kernel_phys: PhysAddr,
    kernel_virt: VirtAddr,
) {
    PhysManager::init(memory_map, identity_base);

    // From now on, we can save logs in memory.
    //Logger::add_sink(Box::new(KernelLogger));

    print!("boot: Memory map provided by bootloder:\n");
    print!("{:^16} {:^16} {}\n", "Address", "Length", "Usage");
    memory_map
        .iter()
        .for_each(|x| print!("{:>16x} {:>16x} {:?}\n", x.address, x.length, x.usage));

    virt::init(kernel_phys, kernel_virt);
}

/// Called after all info from the bootloader has been collected.
/// Initializes all subsystems and starts all servers.
pub fn init(info: &mut BootInfo) {
    print!("boot: Menix v{}\n", env!("CARGO_PKG_VERSION"));

    match info.command_line {
        Some(x) => print!("boot: Command line: \"{x}\"\n"),
        None => print!("boot: Command line is empty.\n"),
    }

    firmware::init(info);
    arch::init::init(info);

    // Load all files.
    if let Some(files) = info.files {
        for file in files {
            print!("boot: Loading \"{}\"\n", file.path);

            let mut table = Arc::new(RwLock::new(PageTable::new(false)));
            let mut task = Task::new(table);
            if let Err(x) = elf::load_from_memory(&mut task, file.data) {
                print!("boot: Failed to load \"{}\": {:?}\n", file.path, x);
            };
        }
    }

    print!("boot: Entering user space...\n");
}
