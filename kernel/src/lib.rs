#![no_std]
#![allow(unused)]
#![allow(clippy::needless_return)]
#![feature(negative_impls)]
#![feature(naked_functions)]
#![feature(allocator_api)]
// Needed for volatile memmove
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(str_from_raw_parts)]
#![feature(new_zeroed_alloc)]
#![feature(cfg_match)]

pub extern crate alloc;
pub extern crate core;

#[macro_use]
pub mod macros;
pub mod arch;
pub mod generic;

use generic::boot::BootInfo;
use generic::cpu::PerCpu;
use generic::memory::{self, PhysAddr, PhysMemory, VirtAddr, virt};

// The boot process is split into 2 stages.
// - `memory_init`: Calls to evaluate the memory map and setup allocator(s).
// - `init`: Calls to initialize the rest of the kernel.

/// Called as soon as a memory map is available.
/// All code ran after this stage can use dynamic allocations.
pub(crate) fn memory_init(
    memory_map: &mut [PhysMemory],
    temp_hhdm: VirtAddr,
    kernel_phys: PhysAddr,
    kernel_virt: VirtAddr,
) {
    memory::init(memory_map, temp_hhdm);
    virt::init(temp_hhdm, kernel_phys, kernel_virt);
}

/// The high-level kernel entry point.
/// Called by [`_start`].
/// Initializes all subsystems and starts all servers.
pub(crate) fn main() -> ! {
    // TODO: Get this information from posix/uname instead
    print!(
        "Menix {}.{}.{}\n",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );

    generic::boot::limine::init();

    // Initialize platform.
    generic::platform::init();
    PerCpu::setup_bsp();
    // Initialize buses.
    generic::bus::init();

    // Finally, load all modules.
    generic::module::init();

    print!("boot: Starting init...\n");

    // Load init.
    // TODO:
    // let init = Process::from_elf(init_path);
    todo!("Load init");
}
