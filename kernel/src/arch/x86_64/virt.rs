use crate::generic::memory::{
    PhysAddr, VirtAddr,
    virt::{PteFlags, mmu::PageTable},
};
use bitflags::bitflags;
use core::arch::asm;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageTableEntry {
    inner: u64,
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let flags = PageFlags::from_bits_truncate(self.inner);
        f.debug_struct("PageTableEntry")
            .field("address", &self.address())
            .field("flags", &flags)
            .finish()
    }
}

/// Masks only the address bits of a PTE.
const ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    struct PageFlags: u64 {
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

impl PageTableEntry {
    pub const fn empty() -> Self {
        return Self { inner: 0 };
    }

    pub const fn new(address: PhysAddr, flags: PteFlags, _level: usize) -> Self {
        let mut result = (address.value() as u64 & ADDR_MASK) | PageFlags::Present.bits();

        if flags.contains(PteFlags::User) {
            result |= PageFlags::UserMode.bits();
        }

        if flags.contains(PteFlags::Directory) {
            result |= PageFlags::ReadWrite.bits();
        } else {
            if flags.contains(PteFlags::Write) {
                result |= PageFlags::ReadWrite.bits();
            }

            if !flags.contains(PteFlags::Exec) {
                result |= PageFlags::ExecuteDisable.bits();
            }

            if flags.contains(PteFlags::Large) {
                result |= PageFlags::Size.bits();
            }
        }

        Self { inner: result }
    }

    pub const fn inner(&self) -> usize {
        self.inner as usize
    }

    pub fn is_present(&self) -> bool {
        PageFlags::from_bits_retain(self.inner).contains(PageFlags::Present)
    }

    pub fn is_directory(&self, level: usize) -> bool {
        level > 0 && !PageFlags::from_bits_retain(self.inner).contains(PageFlags::Size)
    }

    pub fn is_dirty(&self) -> bool {
        PageFlags::from_bits_retain(self.inner).contains(PageFlags::Dirty)
    }

    pub fn address(&self) -> PhysAddr {
        (self.inner & ADDR_MASK).into()
    }
}

pub(in crate::arch) fn flush_tlb(addr: VirtAddr) {
    unsafe {
        asm!("invlpg [{addr}]", addr = in(reg) addr.value());
    }
}

pub(in crate::arch) unsafe fn set_page_table(pt: &PageTable) {
    unsafe {
        asm!("mov cr3, {addr}", addr = in(reg) pt.get_head_addr().value());
    }
}

pub(in crate::arch) const fn get_page_bits() -> usize {
    12
}

pub(in crate::arch) const fn get_max_leaf_level() -> usize {
    2
}

pub(in crate::arch) const fn get_level_bits() -> usize {
    9
}

pub(in crate::arch) const fn get_num_levels() -> usize {
    4
}

pub(in crate::arch) fn get_hhdm_base() -> VirtAddr {
    VirtAddr::new(0xFFFF_8000_0000_0000)
}

pub(in crate::arch) fn get_pfndb_base() -> VirtAddr {
    VirtAddr::new(0xFFFF_A000_0000_0000)
}

pub(in crate::arch) fn get_map_base() -> VirtAddr {
    VirtAddr::new(0xFFFF_C000_0000_0000)
}
