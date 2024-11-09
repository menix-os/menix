// Virtual memory management

pub use crate::arch::VirtManager;
use crate::{
    arch::{CommonPageMap, PageMap, PhysAddr, VirtAddr},
    boot::BootInfo,
    log,
    misc::kernel::{self, LD_KERNEL_START},
    system::error::Errno,
};
use bitflags::bitflags;

use super::pm::PhysManager;

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

pub enum VmLevel {
    /// The smallest page size available.
    Small,
    /// A page size between `Small` and `Large`.
    /// The definition of this value is up to the implementing architecture.
    Medium,
    /// The largest page size available.
    Large,
}

/// Virtual memory manager.
/// Implementations must be called `VirtManager`.
pub trait CommonVirtManager {
    /// Sets a page map to be the current one.
    fn set_page_map(page_map: &PageMap);

    /// Maps a virtual address to physical memory.
    fn map_page(
        page_map: &PageMap,
        phys_addr: PhysAddr,
        virt_addr: VirtAddr,
        prot: VmProt,
        flags: VmFlags,
        level: VmLevel,
    ) -> Result<(), Errno>;

    /// Modifies an existing mapping.
    fn remap_page(
        page_map: &PageMap,
        virt_addr: VirtAddr,
        prot: VmProt,
        flags: VmFlags,
    ) -> Result<(), Errno>;

    /// Destroys an existing mapping.
    fn unmap_page(page_map: &PageMap, virt_addr: VirtAddr) -> Result<(), Errno>;

    /// Translates a mapped address into a physical address.
    fn virt_to_phys(page_map: &PageMap, addr: VirtAddr) -> Result<PhysAddr, Errno>;

    /// Gets the page size in bytes for a page at `level`.
    fn get_page_size(level: VmLevel) -> usize;

    /// Initializes the virtual memory manager.
    /// This function recreates the virtual mappings of the kernel and switches to the kernel-owned page map.
    /// This function must be called after the physical memory allocator has been initialized.
    fn init(info: &BootInfo) {
        // Allocate a page map for the kernel.
        let kernel_map = PageMap::new(None);
        log!("vm: Allocated a new page map for the kernel.\n");

        // Map all physical space.
        // Check for the highest usable physical memory address, so we know how much memory to map.
        let mut highest = 0;
        for entry in info.memory_map.as_ref() {
            let region_end = entry.address + entry.length as PhysAddr;
            if region_end > highest {
                highest = region_end;
            }
        }

        log!("vm: Remapping physical memory to HHDM base.\n");
        let phys_base = PhysManager::phys_base() as VirtAddr;
        for cur in (0..highest).step_by(VirtManager::get_page_size(VmLevel::Large)) {
            VirtManager::map_page(
                &kernel_map,
                cur,
                phys_base + cur,
                VmProt::Read | VmProt::Write,
                VmFlags::None,
                VmLevel::Large,
            );
        }

        unsafe {
            log!("vm: Mapping the text segment.");
            for cur in (kernel::LD_TEXT_START..kernel::LD_TEXT_END)
                .step_by(VirtManager::get_page_size(VmLevel::Small))
            {
                VirtManager::map_page(
                    &kernel_map,
                    cur - LD_KERNEL_START + info.kernel_addr.0,
                    cur,
                    VmProt::Read | VmProt::Exec,
                    VmFlags::None,
                    VmLevel::Small,
                );
            }
            log!("vm: Mapping the data segment.");
            for cur in (kernel::LD_DATA_START..kernel::LD_DATA_END)
                .step_by(VirtManager::get_page_size(VmLevel::Small))
            {
                VirtManager::map_page(
                    &kernel_map,
                    cur - LD_KERNEL_START + info.kernel_addr.0,
                    cur,
                    VmProt::Read | VmProt::Write,
                    VmFlags::None,
                    VmLevel::Small,
                );
            }
            log!("vm: Mapping the rodata segment.");
            for cur in (kernel::LD_RODATA_START..kernel::LD_RODATA_END)
                .step_by(VirtManager::get_page_size(VmLevel::Small))
            {
                VirtManager::map_page(
                    &kernel_map,
                    cur - LD_KERNEL_START + info.kernel_addr.0,
                    cur,
                    VmProt::Read,
                    VmFlags::None,
                    VmLevel::Small,
                );
            }
        }

        todo!();
    }
}
