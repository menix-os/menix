use crate::generic::{cmdline::CmdLine, fbcon::FrameBuffer, memory::PhysAddr};
use alloc::{boxed::Box, string::String, vec::Vec};
use spin::Once;

/// Information passed from the bootloader. Memory is reclaimed after initialization.
#[derive(Default, Debug)]
pub struct BootInfo {
    /// Kernel command line.
    pub command_line: String,
    /// Base address of the RSDT/XSDT ACPI table.
    pub rsdp_addr: Option<PhysAddr>,
    /// Base address of a flattened device tree in memory.
    pub fdt_addr: Option<PhysAddr>,
    /// A framebuffer.
    pub frame_buffer: Option<FrameBuffer>,
    /// Files given to the bootloader.
    pub files: Vec<BootFile>,
}

static BOOT_INFO: Once<BootInfo> = Once::new();

impl BootInfo {
    pub const fn new() -> Self {
        Self {
            command_line: String::new(),
            rsdp_addr: None,
            fdt_addr: None,
            frame_buffer: None,
            files: Vec::new(),
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
#[derive(Default, Clone, Debug)]
pub struct BootFile {
    pub data: Box<[u8]>,
    pub name: String,
    pub command_line: String,
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
