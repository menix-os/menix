use crate::{
    arch::{Cpu, PhysAddr, VirtAddr},
    memory::pm::PhysMemory,
};
use core::sync::atomic::AtomicUsize;

// Boot protocols

// Limine supports x86_64, aarch64, riscv64 and loongarch64
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "loongarch64"
))]
mod limine;

// Kernel entry point.
mod entry;

/// Information passed from the bootloader. Memory is reclaimed after initialization.
#[derive(Default, Debug)]
pub struct BootInfo<'a> {
    /// Kernel command line
    pub command_line: Option<&'a str>,

    /// Files to mount into the VFS.
    pub files: &'a [BootFile<'a>],

    // Physical memory map. Mutable in case the architecture has to modify some entries.
    pub memory_map: &'a mut [PhysMemory],

    /// Physical and virtual address where the kernel was loaded.
    pub kernel_addr: (PhysAddr, VirtAddr),

    /// Base address of a 1:1 physical to virtual mapping.
    pub identity_base: VirtAddr,

    /// Processor information.
    pub smp_info: BootSmpInfo<'a>,

    #[cfg(feature = "sys_acpi")]
    pub rsdp_addr: VirtAddr,
}

/// A file loaded by the bootloader. Memory is reclaimed after initialization.
#[derive(Debug)]
pub struct BootFile<'a> {
    pub data: &'a [u8],
    pub path: &'a str,
}

#[derive(Default, Debug)]
pub struct BootSmpInfo<'a> {
    /// Array of available processors.
    pub processors: &'a [Cpu],

    /// Total active processors.
    pub active_processors: AtomicUsize,

    /// Index of the processor that is being used to boot.
    pub boot_cpu: usize,
}
