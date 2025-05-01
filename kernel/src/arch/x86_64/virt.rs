use super::{consts, irq::TrapFrame};
use crate::generic::{
    self,
    memory::{
        PhysAddr, VirtAddr,
        page::{self, PageFaultInfo, PageFaultKind},
        virt::{self, PageTable, VmFlags, VmLevel},
    },
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

    pub const fn new(address: PhysAddr, flags: VmFlags, level: usize) -> Self {
        let mut result = (address.value() as u64 & ADDR_MASK) | PageFlags::Present.bits();

        if flags.contains(VmFlags::User) {
            result |= PageFlags::UserMode.bits();
        }

        if flags.contains(VmFlags::Directory) {
            result |= PageFlags::ReadWrite.bits();
        } else {
            if flags.contains(VmFlags::Write) {
                result |= PageFlags::ReadWrite.bits();
            }

            if !flags.contains(VmFlags::Exec) {
                result |= PageFlags::ExecuteDisable.bits();
            }

            if flags.contains(VmFlags::Large) {
                result |= PageFlags::Size.bits();
            }
        }

        return Self { inner: result };
    }

    pub const fn inner(&self) -> usize {
        return self.inner as usize;
    }

    pub fn is_present(&self) -> bool {
        return PageFlags::from_bits_retain(self.inner).contains(PageFlags::Present);
    }

    pub fn is_directory(&self, level: usize) -> bool {
        return level > 0 && !PageFlags::from_bits_retain(self.inner).contains(PageFlags::Size);
    }

    pub fn address(&self) -> PhysAddr {
        return (self.inner & ADDR_MASK).into();
    }
}

/// Invalidates a TLB entry cache.
fn flush_tlb(addr: VirtAddr) {
    unsafe {
        asm!("invlpg [{addr}]", addr = in(reg) addr.value());
    }
}

pub unsafe fn set_page_table(addr: PhysAddr) {
    unsafe {
        asm!("mov cr3, {addr}", addr = in(reg) addr.value());
    }
}

pub const fn get_page_bits() -> usize {
    12
}

pub const fn get_max_level() -> VmLevel {
    VmLevel::L3
}

pub const fn get_level_bits() -> usize {
    9
}
