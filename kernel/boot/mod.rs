use super::arch::Processor;
use crate::arch::PhysAddr;

#[cfg(feature = "boot_limine")]
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
    pub kernel_addr: PhysAddr,

    pub phys_map: usize,

    #[cfg(feature = "smp")]
    pub smp_info: BootSmpInfo<'a>,
    // TODO: ACPI, FDT
}

/// A file loaded by the bootloader. Memory is reclaimed after initialization.
pub struct BootFile<'a> {
    pub data: &'a [u8],
    pub path: &'a str,
}

#[derive(Default)]
pub struct BootSmpInfo<'a> {
    /// Array of available processors.
    pub processors: &'a [Processor],

    /// Total active processors.
    pub active_processors: usize,

    /// Index of the processor that is being used to boot.
    pub boot_cpu: usize,
}
