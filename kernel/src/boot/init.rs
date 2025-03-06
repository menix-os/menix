// Kernel initialization.

use core::arch::asm;

use super::BootInfo;
use crate::{
    arch::{self, PerCpu, PhysAddr, VirtAddr},
    generic::{
        self,
        log::{self, KernelLogger, Logger, LoggerSink},
        phys::{PhysManager, PhysMemory},
        virt::{self, PageTable, VmFlags},
    },
};
use alloc::{boxed::Box, string::String, vec::Vec};

// The boot process is split into 3 stages.
// - `early_init`: Very early calls that don't need dynamic memory allocations.
// - `memory_init`: Calls to evaluate the memory map and setup allocators.
// - `init`: Calls to initialize the rest of the kernel.

/// Called as the very first thing during boot. Initializes very basic I/O and temporary features.
pub fn early_init() {
    arch::early_init();
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
    print!("Menix {}\n", env!("CARGO_PKG_VERSION"));

    match info.command_line {
        Some(x) => print!("boot: Command line: \"{x}\"\n"),
        None => print!("boot: Command line is empty.\n"),
    }

    arch::init(info);

    // Load all files.
    if let Some(files) = info.files {
        print!("boot: Got {} loadable files.\n", files.len());
        for file in files {
            // let thread = Thread::new(file.path, );
            // elf::load_from_memory(thread.page_map, file.data);
            //
        }
    }

    todo!();
}
