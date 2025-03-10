use crate::{
    arch::{self, PhysAddr, VirtAddr},
    generic::phys::{PhysManager, PhysMemory},
};

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

/// Information passed from the bootloader. Memory is reclaimed after initialization.
#[derive(Default, Debug)]
pub struct BootInfo<'a> {
    /// Kernel command line.
    pub command_line: Option<&'a str>,
    /// Base address of the RSDT/XSDT ACPI table.
    pub rsdp_addr: Option<PhysAddr>,
    /// Base address of a flattened device tree in memory.
    pub fdt_addr: Option<*const u8>,
    /// Files given to the bootloader.
    pub files: Option<&'a [BootFile<'a>]>,
}

/// A file loaded by the bootloader. Memory is reclaimed after initialization.
#[derive(Default, Clone, Copy, Debug)]
pub struct BootFile<'a> {
    pub data: &'a [u8],
    pub path: &'a str,
}
