use super::memory::{PhysMemory, VirtAddr};
use crate::generic::{cmdline::CmdLine, memory::PhysAddr};
use bootcon::FrameBuffer;
use spin::Once;

pub mod bootcon;

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
    pub command_line: &'static str,
    /// Files given to the bootloader.
    pub files: &'static [BootFile],
    /// Temporary base address for the kernel to access physical memory.
    pub hhdm_address: Option<VirtAddr>,
    /// A list of valid physical memory.
    pub memory_map: &'static [PhysMemory],
    /// The start of the physical kernel address.
    pub kernel_phys: PhysAddr,
    /// The start of the virtual kernel address.
    pub kernel_virt: VirtAddr,
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
            command_line: "",
            files: &[],
            hhdm_address: None,
            memory_map: &[],
            kernel_phys: PhysAddr(0),
            kernel_virt: VirtAddr(0),
            rsdp_addr: None,
            fdt_addr: None,
            framebuffer: None,
        }
    }

    pub fn command_line(&self) -> CmdLine {
        CmdLine::new(&self.command_line)
    }

    pub fn register(self) {
        BOOT_INFO.call_once(|| return self);
    }

    pub fn get() -> &'static Self {
        return BOOT_INFO
            .get()
            .expect("Boot info wasn't set yet! Did you forget to call BootInfo::set()?");
    }
}

/// A file loaded by the bootloader. Memory is reclaimed after initialization.
#[derive(Clone, Copy, Debug)]
pub struct BootFile {
    pub data: &'static [u8],
    pub name: &'static str,
    pub command_line: &'static str,
}

impl BootFile {
    pub const fn new() -> Self {
        Self {
            data: &[],
            name: "",
            command_line: "",
        }
    }
}
