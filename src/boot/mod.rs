use crate::{
    arch::{Cpu, PhysAddr, VirtAddr},
    memory::pm::PhysMemory,
};

#[cfg(all(
    feature = "boot_limine",
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    )
))]
mod limine;

/// Information passed from the bootloader. Memory is reclaimed after initialization.
#[derive(Default)]
pub struct BootInfo<'a> {
    /// Kernel command line
    pub command_line: Option<&'a str>,

    /// Files to mount into the VFS.
    pub files: &'a [BootFile<'a>],

    // Physical memory map.
    pub memory_map: &'a [PhysMemory],

    /// Physical and virtual address where the kernel was loaded.
    pub kernel_addr: (PhysAddr, VirtAddr),

    /// Base address of a 1:1 physical to virtual mapping.
    pub hhdm_base: VirtAddr,

    #[cfg(feature = "smp")]
    pub smp_info: BootSmpInfo<'a>,

    #[cfg(feature = "fw_acpi")]
    pub rsdp_addr: VirtAddr,
}

/// A file loaded by the bootloader. Memory is reclaimed after initialization.
pub struct BootFile<'a> {
    pub data: &'a [u8],
    pub path: &'a str,
}

#[derive(Default)]
pub struct BootSmpInfo<'a> {
    /// Array of available processors.
    pub processors: &'a [Cpu],

    /// Total active processors.
    pub active_processors: usize,

    /// Index of the processor that is being used to boot.
    pub boot_cpu: usize,
}
