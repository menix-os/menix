use crate::{
    arch::VirtAddr,
    generic::virt::{GenericPageMap, VmFlags, VmProt},
};
use bitflags::bitflags;
use core::arch::asm;
use portal::error::Error;
use spin::Mutex;

/// Masks only the address bits of a virtual address.
const ADDR_MASK: VirtAddr = 0x00007FFFFFFFF000;

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

impl PageFlags {
    fn to_x86_flags(prot: VmProt, flags: VmFlags) -> Self {
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
    head: *mut usize,
}

impl GenericPageMap for PageMap {
    fn map(
        &mut self,
        virt: VirtAddr,
        phys: super::PhysAddr,
        prot: VmProt,
        flags: VmFlags,
    ) -> Result<(), Error> {
        todo!()
    }

    fn unmap(&mut self, virt: VirtAddr) -> Result<(), Error> {
        todo!()
    }

    fn remap(&mut self, virt: VirtAddr, prot: VmProt, flags: VmFlags) -> Result<(), Error> {
        todo!()
    }

    fn is_mapped(&self, virt: VirtAddr) -> bool {
        todo!()
    }
}

/// Invalidates a TLB entry cache.
fn flush_tlb(addr: VirtAddr) {
    unsafe {
        asm!("invlpg [{addr}]", addr = in(reg) addr);
    }
}
