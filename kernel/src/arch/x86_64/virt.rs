use super::PhysAddr;
use crate::{
    arch::VirtAddr,
    generic::virt::{PageTable, VmFlags},
};
use bitflags::bitflags;
use core::arch::asm;
use portal::error::Error;
use spin::Mutex;

#[repr(transparent)]
pub struct PageTableEntry {
    inner: PhysAddr,
}

/// Masks only the address bits of a PTE.
const ADDR_MASK: PhysAddr = 0x000FFFFFFFFFF000;

bitflags! {
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
    pub fn new(address: PhysAddr, flags: VmFlags) -> Self {
        let mut result = (address & ADDR_MASK) | PageFlags::Present.bits();

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
}

/// Invalidates a TLB entry cache.
fn flush_tlb(addr: VirtAddr) {
    unsafe {
        asm!("invlpg [{addr}]", addr = in(reg) addr);
    }
}

pub unsafe fn set_page_table<const K: bool>(page_table: &PageTable<K>) {
    unsafe {
        let table = *page_table.head.lock();
        asm!("mov cr3, {addr}", addr = in(reg) table);
    }
}
