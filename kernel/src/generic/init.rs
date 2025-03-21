// Kernel initialization.

use crate::{
    arch::{self, PhysAddr, VirtAddr},
    boot::BootInfo,
    generic::{
        self,
        memory::{self, PhysMemory},
        virt,
    },
};
use alloc::boxed::Box;

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
    temp_hhdm: VirtAddr,
    kernel_phys: PhysAddr,
    kernel_virt: VirtAddr,
) {
    print!("boot: Memory map provided by bootloder:\n");
    print!("{:^16} {:^16} {}\n", "Address", "Length", "Usage");
    memory_map
        .iter()
        .for_each(|x| print!("{:>16x} {:>16x} {:?}\n", x.address, x.length, x.usage));

    memory::init(memory_map, temp_hhdm);
    virt::init(temp_hhdm, kernel_phys, kernel_virt);
}

/// Called after all info from the bootloader has been collected.
/// Initializes all subsystems and starts all servers.
pub(crate) fn init(info: &mut BootInfo) {
    print!("Menix {}\n", crate::MENIX_VERSION);

    match info.command_line {
        Some(x) => print!("boot: Command line: \"{x}\"\n"),
        None => print!("boot: Command line is empty.\n"),
    }

    generic::firmware::init(info);
    arch::init::init(info);
    generic::module::init();

    // Load all modules.
    if let Some(files) = info.files {
        for file in files {
            print!("boot: Loading module \"{}\"\n", file.path);
            // TODO: Load the modules :^)
        }
    }

    print!("boot: Starting init...\n");

    // Load init.
    // TODO:
    // let init = Process::from_elf(init_path);
    todo!();
}
