use core::arch::asm;

use super::{misc::align_up, phys::PhysManager};
use crate::arch::{self, PageTableEntry, PhysAddr, VirtAddr, get_page_size};
use alloc::boxed::Box;
use bitflags::bitflags;
use portal::error::Error;
use spin::Mutex;

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
    pub struct VmFlags: usize {
        const None = 0x00;
        /// Page can be read from.
        const Read = 0x01;
        /// Page can be written to.
        const Write = 0x02;
        /// Page has executable code.
        const Exec = 0x04;
        /// Page can be accessed by the user.
        const User = 0x08;
        /// Page is a large page.
        const Large = 0x10;
    }
}

/// Represents a virtual address space.
pub struct PageTable<const K: bool> {
    pub head: Mutex<PhysAddr>,
}

impl<const K: bool> PageTable<K> {
    pub fn new() -> Self {
        return Self {
            head: Mutex::new(PhysManager::alloc_zeroed(1)),
        };
    }

    /// Gets the page table entry pointed to by `virt`.
    /// Allocates new levels if necessary and requested.
    /// `length`: The amount of bytes this PTE should be able to fit. This helps allocate more efficient entries.
    pub fn get_pte(&self, virt: VirtAddr, allocate: bool) -> Option<&mut PageTableEntry> {
        let mut head = self.head.lock();

        let mut current_head = *head;
        let mut index = 0;

        // Traverse the page table (from highest to lowest level).
        for level in (0..PageTableEntry::get_levels()).rev() {
            // Create a mask for the address part of the PTE, e.g. 0x1ff for 9 bits.
            let addr_bits = (usize::MAX
                >> (0usize.trailing_zeros() as usize - PageTableEntry::get_level_bits()));

            // Determine the shift for the appropriate level, e.g. x << (12 + (9 * level)).
            let addr_shift =
                PageTableEntry::get_page_bits() + (PageTableEntry::get_level_bits() * level);

            // Get the index for this level by masking the relevant address part.
            index = (virt >> addr_shift) & addr_bits;

            // The last level is used to access the actual PTE, so break the loop then.
            if level == 0 {
                break;
            }

            unsafe {
                let pte =
                    (PhysManager::direct_access(current_head) as *mut PageTableEntry).add(index);

                let mut pte_flags = VmFlags::Read
                    | VmFlags::Write
                    | VmFlags::Exec
                    | if !K { VmFlags::User } else { VmFlags::None };

                if (*pte).is_present() {
                    if (*pte).is_large() {
                        pte_flags |= VmFlags::Large;
                        break;
                    } else {
                        current_head = (*pte).address();
                    }
                } else {
                    // PTE isn't present, we have to allocate a new level.
                    if !allocate {
                        return None;
                    }

                    let next_head = PhysManager::alloc_zeroed(1);
                    *pte = PageTableEntry::new(next_head, pte_flags);
                    current_head = next_head;
                }
            }
        }

        unsafe {
            let pte = (PhysManager::direct_access(current_head) as *mut PageTableEntry).add(index);
            return Some(pte.as_mut().unwrap());
        }
    }

    /// Maps a single page in this address space.
    pub fn map_single(
        &mut self,
        virt: VirtAddr,
        phys: PhysAddr,
        flags: VmFlags,
    ) -> Result<(), Error> {
        let pte = self.get_pte(virt, true).unwrap();
        *pte = PageTableEntry::new(phys, flags);
        return Ok(());
    }

    /// Unmaps a page from this address space.
    pub fn unmap(&mut self, virt: VirtAddr) -> Result<(), Error> {
        Ok(())
    }

    /// Checks if the address (may be unaligned) is mapped in this address space.
    pub fn is_mapped(&self, virt: VirtAddr) -> bool {
        let pte = self.get_pte(virt, false);
        match pte {
            Some(x) => x.is_present(),
            None => {
                return false;
            }
        }
    }
}

pub static KERNEL_TABLE: Mutex<Option<PageTable<true>>> = Mutex::new(None);

/// Initialize the kernel page table and switch to it.
pub fn init(kernel_phys: PhysAddr, kernel_virt: VirtAddr) {
    let mut table = PageTable::<true>::new();

    unsafe {
        let text_start = (&LD_TEXT_START as *const u8) as VirtAddr;
        let text_end = (&LD_TEXT_END as *const u8) as VirtAddr;
        let rodata_start = (&LD_RODATA_START as *const u8) as VirtAddr;
        let rodata_end = (&LD_RODATA_END as *const u8) as VirtAddr;
        let data_start = (&LD_DATA_START as *const u8) as VirtAddr;
        let data_end = (&LD_DATA_END as *const u8) as VirtAddr;
        let offset = (&LD_KERNEL_START as *const u8) as VirtAddr;

        for virt in (text_start..text_end).step_by(get_page_size()) {
            table
                .map_single(
                    virt,
                    virt - offset + kernel_phys,
                    VmFlags::Read | VmFlags::Exec,
                )
                .expect("Unable to map the text segment!");
        }

        for virt in (rodata_start..rodata_end).step_by(get_page_size()) {
            table
                .map_single(virt, virt - offset + kernel_phys, VmFlags::Read)
                .expect("Unable to map the rodata segment!");
        }

        for virt in (data_start..data_end).step_by(get_page_size()) {
            table
                .map_single(
                    virt,
                    virt - offset + kernel_phys,
                    VmFlags::Read | VmFlags::Write,
                )
                .expect("Unable to map the data segment!");
        }

        // TODO: Needs large page support.
        for virt in (0x0..0x10000000).step_by(get_page_size()) {
            table
                .map_single(
                    0xffff800000000000 + virt,
                    virt,
                    VmFlags::Read | VmFlags::Write,
                )
                .expect("Unable to map the HHDM!");
        }

        arch::set_page_table(&table);

        let mut table_opt = KERNEL_TABLE.lock();
        *table_opt = Some(table);
    }
}

// Symbols defined in the linker script so we can map ourselves in our page table.
unsafe extern "C" {
    unsafe static LD_KERNEL_START: u8;
    unsafe static LD_KERNEL_END: u8;
    unsafe static LD_TEXT_START: u8;
    unsafe static LD_TEXT_END: u8;
    unsafe static LD_RODATA_START: u8;
    unsafe static LD_RODATA_END: u8;
    unsafe static LD_DATA_START: u8;
    unsafe static LD_DATA_END: u8;
}
