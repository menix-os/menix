use super::error::Error;
use crate::arch::{PhysAddr, VirtAddr};
use bitflags::bitflags;

// User constants
const USER_STACK_SIZE: usize = 0x200000;
const USER_STACK_BASE: usize = 0x00007F0000000000;
const USER_MAP_BASE: usize = 0x0000600000000000;

// Kernel constants
const KERNEL_STACK_SIZE: usize = 0x20000;
const MAP_BASE: usize = 0xFFFF90000000000;
const MEMORY_BASE: usize = 0xFFFFA0000000000;
const MODULE_BASE: usize = 0xFFFFB0000000000;

bitflags! {
    /// Page protection flags.
    pub struct VmProt: usize {
        const None = 0x00;
        /// Page can be read from.
        const Read = 0x01;
        /// Page can be written to.
        const Write = 0x02;
        /// Page has executable code.
        const Exec = 0x04;
    }

    /// Other page flags.
    pub struct VmFlags: usize {
        const None = 0x00;
        /// Page can be accessed by the user.
        const User = 0x01;
    }
}

/// Represents a virtual address space.
pub trait GenericPageMap {
    /// Maps a virtual page in this address space.
    fn map(
        &mut self,
        virt: VirtAddr,
        phys: PhysAddr,
        prot: VmProt,
        flags: VmFlags,
    ) -> Result<(), Error>;

    /// Unmaps a virtual page from this address space.
    fn unmap(&mut self, virt: VirtAddr) -> Result<(), Error>;

    /// Modifies the flags of an existing mapping in this address space.
    fn remap(&mut self, virt: VirtAddr, prot: VmProt, flags: VmFlags) -> Result<(), Error>;

    /// Checks if the address (may be unaligned) is mapped in this address space.
    fn is_mapped(&self, virt: VirtAddr) -> bool;
}

// Symbols defined in the linker script so we can map ourselves in our memory map.
unsafe extern "C" {
    pub static LD_KERNEL_START: VirtAddr;
    pub static LD_KERNEL_END: VirtAddr;
    pub static LD_TEXT_START: VirtAddr;
    pub static LD_TEXT_END: VirtAddr;
    pub static LD_RODATA_START: VirtAddr;
    pub static LD_RODATA_END: VirtAddr;
    pub static LD_DATA_START: VirtAddr;
    pub static LD_DATA_END: VirtAddr;
}
