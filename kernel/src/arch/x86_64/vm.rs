// Virtual memory management

use super::{PhysAddr, PhysManager, VirtAddr};
use crate::{
    arch::CommonPageMap,
    boot::BootInfo,
    memory::{
        pm::CommonPhysManager,
        vm::{CommonVirtManager, ProtFlags},
    },
    system::error::Errno,
};
use alloc::boxed::Box;
use bitflags::bitflags;
use core::{arch::asm, ptr::null_mut};
use spin::Mutex;

pub struct VirtManager {
    /// Memory mapped lower physical memory.
    pub(crate) phys_base: VirtAddr,
    /// Page map used for the kernel.
    pub(crate) kernel_map: Option<Box<PageMap>>,
    /// Start of foreign mappings.
    pub(crate) foreign_base: VirtAddr,
    /// If we can use the Supervisor Mode Access Prevention to run vm_hide_user() and vm_show_user()
    pub(crate) can_smap: bool,
}

static VMM: Mutex<VirtManager> = Mutex::new(VirtManager {
    phys_base: 0,
    kernel_map: None,
    foreign_base: 0,
    can_smap: false,
});

impl CommonVirtManager for VirtManager {
    unsafe fn init(info: &BootInfo) {
        todo!()
    }

    fn map_page(
        page_map: &PageMap,
        phys_addr: PhysAddr,
        virt_addr: VirtAddr,
        flags: ProtFlags,
    ) -> Result<(), Errno> {
        todo!()
    }

    fn remap_page(page_map: &PageMap, virt_addr: VirtAddr, flags: ProtFlags) -> Result<(), Errno> {
        todo!()
    }

    fn unmap_page(page_map: &PageMap, virt_addr: VirtAddr) -> Result<(), Errno> {
        todo!()
    }

    fn virt_to_phys(addr: VirtAddr) -> Result<PhysAddr, Errno> {
        todo!()
    }

    unsafe fn set_page_map(page_map: &PageMap) {
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
    fn from_posix(page_map: &PageMap, prot: ProtFlags) -> Self {
        let mut result = PageFlags::None;

        match &VMM.lock().kernel_map {
            Some(x) => {
                if *page_map != **x {
                    result |= PageFlags::UserMode
                }
            }
            None => result |= PageFlags::UserMode,
        }
        if prot.contains(ProtFlags::Write) {
            result |= PageFlags::ReadWrite;
        }
        if prot.contains(ProtFlags::Exec) {
            result |= PageFlags::ExecuteDisable;
        }

        return result;
    }
}

pub struct PageMap {
    head: Mutex<&'static mut [usize; 512]>,
}

impl PartialEq for PageMap {
    fn eq(&self, other: &Self) -> bool {
        self.head.lock().as_ptr() == other.head.lock().as_ptr()
    }
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
        *top.add(idx) = next_level | PageFlags::Present.bits() | PageFlags::ReadWrite.bits();

        return Some(PhysManager::phys_base().byte_add(next_level as usize) as *mut VirtAddr);
    }
}

impl CommonPageMap for PageMap {
    fn new(copy_from: Option<&Self>) -> Self {
        let result = unsafe {
            let pt = PhysManager::phys_base().add(PhysManager::alloc(1) as usize) as *mut usize;
            Self {
                head: Mutex::new(&mut *(pt as *mut [usize; 512])),
            }
        };

        if let Some(x) = copy_from {
            let mut page = result.head.lock();
            let old_page = x.head.lock();
            for i in 256..512 {
                page[i] = old_page[i]
            }
        }

        return result;
    }

    fn fork(source: &Self) -> Self {
        todo!()
    }
}

/// Invalidates a TLB entry cache.
unsafe extern "C" fn flush_tlb(addr: VirtAddr) {
    unsafe {
        asm!("invlpg [{addr}]", addr = in(reg) addr);
    }
}
