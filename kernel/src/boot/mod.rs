use crate::{
    arch::{self, PerCpu, PhysAddr, VirtAddr},
    generic::{
        phys::{PhysManager, PhysMemory},
        schedule::PreemptionGuard,
    },
};

/// Information passed from the bootloader. Memory is reclaimed after initialization.
#[derive(Default, Debug)]
pub struct BootInfo<'a> {
    /// Kernel command line.
    pub command_line: Option<&'a str>,

    /// Base address of the RSDT/XSDT ACPI table.
    pub rsdp_addr: PhysAddr,

    /// Files given to the bootloader.
    pub files: Option<&'a [BootFile<'a>]>,
}

/// A file loaded by the bootloader. Memory is reclaimed after initialization.
#[derive(Debug)]
pub struct BootFile<'a> {
    pub data: &'a [u8],
    pub path: &'a str,
}

/// Called as the very first thing during boot. Initializes very basic I/O and temporary features.
pub fn early_init() {
    arch::early_init();
}

/// Called as soon as a memory map is available.
/// All code ran after this stage can use dynamic allocations.
pub fn memory_init(
    memory_map: &mut [PhysMemory],
    identity_base: VirtAddr,
    kernel_addr: (PhysAddr, VirtAddr),
) {
    PhysManager::init(memory_map, identity_base);
}

fn get() -> PerCpu {
    let token = PreemptionGuard::get();
    return PerCpu {
        id: 0,
        kernel_stack: 0,
        user_stack: 0,
        thread: None,
        ticks_active: 0,
        enabled: true,
    };
}

/// Called after all info from the bootloader has been collected.
/// Initializes all subsystems and starts all servers.
pub fn init(info: &mut BootInfo) {
    let cpu = get();
    arch::init(info);

    print!(
        "Menix {}\n",
        option_env!("CARGO_PKG_VERSION").unwrap_or("???")
    );

    match info.command_line {
        Some(x) => print!("boot: Command line: \"{x}\"\n"),
        None => print!("boot: Command line is empty.\n"),
    }

    // Load all servers.
    if let Some(files) = info.files {
        print!("boot: Have {} loadable files.\n", files.len());
        for file in files {
            //generic::elf::load_from_memory(file.data);
        }
    }

    todo!();
}

// Boot method selection. Limine is the default method.
#[cfg(all(
    feature = "boot_limine",
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    )
))]
pub mod limine;
