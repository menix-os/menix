// Virtual memory management

use super::{PhysAddr, VirtAddr};
use crate::{
    arch::CommonPageMap,
    memory::{
        pm::PhysManager,
        vm::{CommonVirtManager, VmFlags, VmLevel, VmProt},
    },
    system::error::Errno,
};
use alloc::boxed::Box;
use bitflags::bitflags;
use core::{arch::asm, ptr::null_mut};
use spin::Mutex;

pub struct VirtManager {
    /// Memory mapped lower physical memory.
    pub phys_base: VirtAddr,
    /// Page map used for the kernel.
    pub kernel_map: Option<PageMap>,
    /// If we can use the Supervisor Mode Access Prevention.
    pub can_smap: bool,
}

static VMM: Mutex<VirtManager> = Mutex::new(VirtManager {
    phys_base: 0,
    kernel_map: None,
    can_smap: false,
});

impl CommonVirtManager for VirtManager {
    fn map_page(
        page_map: &PageMap,
        phys_addr: PhysAddr,
        virt_addr: VirtAddr,
        prot: VmProt,
        flags: VmFlags,
        level: VmLevel,
    ) -> Result<(), Errno> {
        todo!()
    }

    fn remap_page(
        page_map: &PageMap,
        virt_addr: VirtAddr,
        prot: VmProt,
        flags: VmFlags,
    ) -> Result<(), Errno> {
        todo!()
    }

    fn unmap_page(page_map: &PageMap, virt_addr: VirtAddr) -> Result<(), Errno> {
        todo!()
    }

    fn virt_to_phys(page_map: &PageMap, addr: VirtAddr) -> Result<PhysAddr, Errno> {
        todo!()
    }

    fn get_page_size(level: VmLevel) -> usize {
        match level {
            VmLevel::Small => 0x1000,     // 4 KiB
            VmLevel::Medium => 0x200000,  // 2 MiB
            VmLevel::Large => 0x40000000, // 1 GiB
        }
    }

    fn set_page_map(page_map: &PageMap) {
        unsafe {
            let buf = page_map.head.lock();
            asm!("mov cr3, {address}", address = in(reg) buf.as_ptr());
        }
    }
}

bitflags! {
    pub struct PageFlags: VirtAddr {
        const None = 0;
        const Present = 1 << 0;
        const ReadWrite = 1 << 1;
        const UserMode = 1 << 2;
        const WriteThrough = 1 << 3;
        const CacheDisable = 1 << 4;
        const Accessed = 1 << 5;
        const Dirty = 1 << 6;
        const Size = 1 << 7;
        const Global = 1 << 8;
        const Available = 1 << 9;
        const AttributeTable = 1 << 10;
        const ExecuteDisable = 1 << 63;
    }
}

/// Masks only the address bits of a virtual address.
const ADDR_MASK: VirtAddr = 0x0000FFFFFFFFF000;

impl PageFlags {
    fn to_x86_flags(page_map: &PageMap, prot: VmProt, flags: VmFlags) -> Self {
        let mut result = PageFlags::None;

        if flags.contains(VmFlags::User) {
            result |= PageFlags::UserMode;
        }
        if prot.contains(VmProt::Write) {
            result |= PageFlags::ReadWrite;
        }
        if !prot.contains(VmProt::Exec) {
            result |= PageFlags::ExecuteDisable;
        }

        return result;
    }
}

pub struct PageMap {
    head: Mutex<&'static mut [usize; 512]>,
}

/// Returns the next level of the current page map level. Optionally allocates a page.
unsafe fn traverse(top: *mut VirtAddr, idx: usize, allocate: bool) -> Option<*mut VirtAddr> {
    let vmm = VMM.lock();
    unsafe {
        // If we have allocated the next level, filter the address as offset and return the level.
        if *top.add(idx) & PageFlags::Present.bits() != 0 {
            return Some(
                PhysManager::phys_base().byte_add((*top.add(idx) & ADDR_MASK) as usize)
                    as *mut VirtAddr,
            );
        }

        // If we don't want to allocate a page, but there was no page present, we can't do anything here.
        if !allocate {
            return None;
        }

        // Allocate the next level (will contain `PAGE_SIZE/sizeof(u64) == 512` entries).
        let next_level = PhysManager::alloc_zeroed(1);
        // Mark the next level as present so we don't allocate again.
        *top.add(idx) = next_level | (PageFlags::Present | PageFlags::ReadWrite).bits();

        return Some(PhysManager::phys_base().byte_add(next_level as usize) as *mut VirtAddr);
    }
}

impl CommonPageMap for PageMap {
    fn new() -> Self {
        let mut result = unsafe {
            let pt =
                PhysManager::phys_base().byte_add(PhysManager::alloc(1) as usize) as *mut usize;
            Self {
                head: Mutex::new(&mut *(pt as *mut [usize; 512])),
            }
        };

        // If we already have a kernel map, copy the PDEs of the upper half into the new page map.
        if let Some(kernel_map) = &VMM.lock().kernel_map {
            let mut page = &mut result.head.lock();
            let kernel_head = kernel_map.head.lock();
            for i in 256..512 {
                page[i] = kernel_head[i];
            }
        }

        return result;
    }

    fn fork(source: &Self) -> Self {
        todo!()
    }
}

/// Invalidates a TLB entry cache.
fn flush_tlb(addr: VirtAddr) {
    unsafe {
        asm!("invlpg [{addr}]", addr = in(reg) addr);
    }
}
