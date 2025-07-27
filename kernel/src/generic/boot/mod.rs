use super::{
    fbcon::FrameBuffer,
    memory::VirtAddr,
    util::{once::Once, spin_mutex::SpinMutex},
};
use crate::generic::{cmdline::CmdLine, memory::PhysAddr};

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
#[derive(Debug)]
pub struct BootInfo {
    /// Kernel command line.
    pub command_line: CmdLine<'static>,
    /// Files given to the bootloader.
    pub files: &'static [BootFile],
    /// Base address for the kernel to access physical memory.
    pub hhdm_address: Option<VirtAddr>,
    /// How many levels the page table has.
    pub paging_level: Option<usize>,
    /// A list of valid physical memory.
    pub memory_map: SpinMutex<&'static mut [PhysMemory]>,
    /// The start of the physical kernel address.
    pub kernel_phys: Option<PhysAddr>,
    /// The start of the virtual kernel address.
    pub kernel_virt: Option<VirtAddr>,
    /// Base address of the RSDT/XSDT ACPI table.
    pub rsdp_addr: Option<PhysAddr>,
    /// Base address of a flattened device tree in memory.
    pub fdt_addr: Option<PhysAddr>,
    /// Early framebuffer if it exists.
    pub framebuffer: Option<FrameBuffer>,
}

static BOOT_INFO: Once<BootInfo> = Once::new();

impl BootInfo {
    pub const fn new() -> Self {
        Self {
            command_line: CmdLine::new(""),
            files: &[],
            hhdm_address: None,
            paging_level: None,
            memory_map: SpinMutex::new(&mut []),
            kernel_phys: None,
            kernel_virt: None,
            rsdp_addr: None,
            fdt_addr: None,
            framebuffer: None,
        }
    }

    pub fn register(self) {
        unsafe { BOOT_INFO.init(self) };
    }

    pub fn get() -> &'static Self {
        return BOOT_INFO.get();
    }
}

/// A file loaded by the bootloader. Memory is reclaimed after initialization.
#[derive(Clone, Copy, Debug)]
pub struct BootFile {
    pub data: &'static [u8],
    pub name: &'static str,
}

impl BootFile {
    pub const fn new() -> Self {
        Self {
            data: &[],
            name: "",
        }
    }
}

/// Describes a region of physical memory.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PhysMemory {
    /// Start address of the memory region.
    pub address: PhysAddr,
    /// Length of the memory region in bytes.
    pub length: usize,
}

impl PhysMemory {
    pub const fn empty() -> Self {
        Self {
            address: PhysAddr::null(),
            length: 0,
        }
    }

    pub const fn new(address: PhysAddr, length: usize) -> Self {
        if length == 0 {
            panic!("Can't construct a PhysMemory descriptor with empty size!");
        }
        Self { address, length }
    }
}
