use super::{PhysAddr, consts, schedule::Context};
use crate::{
    arch::VirtAddr,
    generic::virt::{self, PageFaultInfo, PageTable, VmFlags},
};
use bitflags::bitflags;
use core::arch::asm;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageTableEntry {
    inner: PhysAddr,
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
const ADDR_MASK: PhysAddr = 0x000FFFFFFFFFF000;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct PageFlags: PhysAddr {
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

    pub const fn new(address: PhysAddr, flags: VmFlags) -> Self {
        let mut result = (address & ADDR_MASK) | PageFlags::Present.bits();

        if flags.contains(VmFlags::Directory) {
            result |= PageFlags::ReadWrite.bits();
        }

        if flags.contains(VmFlags::User) {
            result |= PageFlags::UserMode.bits();
        }

        if flags.contains(VmFlags::Write) {
            result |= PageFlags::ReadWrite.bits();
        }

        if !flags.contains(VmFlags::Exec) {
            result |= PageFlags::ExecuteDisable.bits();
        }

        if flags.contains(VmFlags::Large) {
            result |= PageFlags::Size.bits();
        }

        return Self { inner: result };
    }

    pub const fn inner(&self) -> usize {
        return self.inner;
    }

    pub fn is_present(&self) -> bool {
        return PageFlags::from_bits_retain(self.inner).contains(PageFlags::Present);
    }

    pub fn is_large(&self) -> bool {
        return PageFlags::from_bits_retain(self.inner).contains(PageFlags::Size);
    }

    pub fn address(&self) -> PhysAddr {
        return self.inner & ADDR_MASK;
    }

    pub const fn get_levels() -> usize {
        return 4;
    }

    pub const fn get_level_bits() -> usize {
        return 9;
    }

    pub const fn get_page_bits() -> usize {
        return 12;
    }

    pub const fn get_hhdm_addr() -> usize {
        return 0xffff_8000_0000_0000;
    }

    pub const fn get_hhdm_size() -> usize {
        return 0x0000_0100_0000_0000;
    }

    pub const fn get_hhdm_level() -> usize {
        return 2;
    }
}

/// Invalidates a TLB entry cache.
fn flush_tlb(addr: VirtAddr) {
    unsafe {
        asm!("invlpg [{addr}]", addr = in(reg) addr);
    }
}

pub unsafe fn set_page_table(page_table: &PageTable) {
    let table = page_table.head.lock().as_ptr();
    unsafe {
        asm!("mov cr3, {addr}", addr = in(reg) table as VirtAddr - PageTableEntry::get_hhdm_addr());
    }
}

pub unsafe fn page_fault_handler(context: *const Context) -> *const Context {
    let mut cr2 = 0usize;
    unsafe {
        asm!("mov {cr2}, cr2", cr2 = out(reg) cr2);

        let info = PageFaultInfo {
            is_user: (*context).cs & consts::CPL_USER as u64 == consts::CPL_USER as u64,
            ip: (*context).rip as usize,
            addr: cr2,
        };
        return virt::page_fault_handler(context.as_ref().unwrap(), &info);
    }
}
