use super::{PhysAddr, schedule::Context};
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
        const Valid = 1 << 0;
        const Read = 1 << 1;
        const Write = 1 << 2;
        const Execute = 1 << 3;
        const User = 1 << 4;
        const Global = 1 << 5;
    }
}

impl PageTableEntry {
    pub const fn empty() -> Self {
        return Self { inner: 0 };
    }

    pub const fn new(address: PhysAddr, flags: VmFlags) -> Self {
        let mut result = address & ADDR_MASK | PageFlags::Valid.bits();

        if flags.contains(VmFlags::User) {
            result |= PageFlags::User.bits();
        }

        if !flags.contains(VmFlags::Directory) {
            if flags.contains(VmFlags::Read) {
                result |= PageFlags::Read.bits();
            }

            if flags.contains(VmFlags::Write) {
                result |= PageFlags::Write.bits();
            }

            if flags.contains(VmFlags::Exec) {
                result |= PageFlags::Execute.bits();
            }
        }

        return Self { inner: result };
    }

    pub const fn inner(&self) -> usize {
        return self.inner;
    }

    pub fn is_present(&self) -> bool {
        return PageFlags::from_bits_retain(self.inner).contains(PageFlags::Valid);
    }

    pub fn is_directory(&self) -> bool {
        return !PageFlags::from_bits_retain(self.inner)
            .contains(PageFlags::Read | PageFlags::Write | PageFlags::Execute);
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

/// The SATP MODE field.
#[repr(u8)]
enum SatpMode {
    Bare = 0,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
    Sv64 = 11,
}

pub unsafe fn set_page_table(page_table: &PageTable) {
    let table = page_table.head.lock().as_ptr();

    let mode = SatpMode::Sv48 as u64;
    let asid = 0u64;
    let ppn = (table as VirtAddr - PageTableEntry::get_hhdm_addr()) as u64;

    let satp = ((mode & 0xf) << 60) | ((asid & 0xff) << 44) | ((ppn >> 12) & 0xf_ffff_ffff_ffff);

    unsafe {
        asm!("csrw satp, {addr}", addr = in(reg) satp);
    }
}

pub unsafe fn page_fault_handler(context: *const Context) -> *const Context {
    unsafe {
        let info = PageFaultInfo {
            is_user: todo!(),
            ip: todo!(),
            addr: todo!(),
        };
        return virt::page_fault_handler(context.as_ref().unwrap(), &info);
    }
}
