use super::{misc::align_up, phys::PhysManager};
use crate::arch::{self, PhysAddr, VirtAddr, schedule::Context, virt::PageTableEntry};
use alloc::boxed::Box;
use bitflags::bitflags;
use core::{arch::asm, ptr::slice_from_raw_parts};
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
    #[derive(Copy, Clone)]
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
#[derive(Debug)]
pub struct PageTable {
    pub head: Mutex<PhysAddr>,
    pub is_user: bool,
}

impl PageTable {
    pub fn new(is_user: bool) -> Self {
        return Self {
            head: Mutex::new(
                PhysManager::alloc_zeroed(1)
                    .expect("Can't allocate a new page table, out of memory"),
            ),
            is_user,
        };
    }

    /// Gets the page table entry pointed to by `virt`.
    /// Allocates new levels if necessary and requested.
    /// `target_level`: The level to get for the PTE. Use 0 if you're unsure.
    /// `length`: The amount of bytes this PTE should be able to fit. This helps allocate more efficient entries.
    pub fn get_pte(
        &self,
        virt: VirtAddr,
        allocate: bool,
        target_level: usize,
    ) -> Option<&mut PageTableEntry> {
        let mut head = self.head.lock();

        let mut current_head = *head;
        let mut index = 0;
        let mut do_break = false;

        if target_level >= PageTableEntry::get_levels() - 1 {
            return None;
        }

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
            if level <= target_level || do_break {
                break;
            }

            unsafe {
                let pte =
                    (PhysManager::direct_access(current_head) as *mut PageTableEntry).add(index);

                let mut pte_flags = VmFlags::Read
                    | VmFlags::Write
                    | VmFlags::Exec
                    | if !self.is_user {
                        VmFlags::User
                    } else {
                        VmFlags::None
                    };

                if (*pte).is_present() {
                    // If this PTE is a large page, it already contains the final address. Don't continue.
                    if (*pte).is_large() {
                        pte_flags |= VmFlags::Large;

                        // If the caller wanted to go further than this, tell them that it's not possible.
                        if level != target_level {
                            return None;
                        }

                        do_break = true;
                    } else {
                        // If the PTE is not large, go one level deeper.
                        current_head = (*pte).address();
                    }
                    *pte = PageTableEntry::new((*pte).address(), pte_flags);
                } else {
                    // PTE isn't present, we have to allocate a new level.
                    if !allocate {
                        return None;
                    }

                    let next_head = PhysManager::alloc_zeroed(1)?;
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
        level: usize,
    ) -> Result<(), Error> {
        let pte = self.get_pte(virt, true, level).ok_or(Error::NotFound)?;
        *pte = PageTableEntry::new(
            phys,
            flags
                | if level != 0 {
                    VmFlags::Large
                } else {
                    VmFlags::None
                },
        );
        return Ok(());
    }

    /// Maps a range of consecutive memory in this address space.
    pub fn map_range(
        &mut self,
        virt: VirtAddr,
        phys: PhysAddr,
        flags: VmFlags,
        level: usize,
        length: usize,
    ) -> Result<(), Error> {
        let step =
            1 << (PageTableEntry::get_page_bits() + (level * PageTableEntry::get_level_bits()));
        for offset in (0..length).step_by(step) {
            self.map_single(virt + offset, phys + offset, flags, level)?;
        }
        return Ok(());
    }

    /// Unmaps a page from this address space.
    pub fn unmap(&mut self, virt: VirtAddr) -> Result<(), Error> {
        Ok(())
    }

    /// Checks if the address (may be unaligned) is mapped in this address space.
    pub fn is_mapped(&self, virt: VirtAddr, level: usize) -> bool {
        let pte = self.get_pte(virt, false, level);
        match pte {
            Some(x) => x.is_present(),
            None => {
                return false;
            }
        }
    }
}

pub static KERNEL_PAGE_TABLE: Mutex<Option<PageTable>> = Mutex::new(None);

/// Initialize the kernel page table and switch to it.
pub fn init(kernel_phys: PhysAddr, kernel_virt: VirtAddr) {
    let mut table = PageTable::new(true);

    unsafe {
        let text_start = &raw const LD_TEXT_START as VirtAddr;
        let text_end = &raw const LD_TEXT_END as VirtAddr;
        let rodata_start = &raw const LD_RODATA_START as VirtAddr;
        let rodata_end = &raw const LD_RODATA_END as VirtAddr;
        let data_start = &raw const LD_DATA_START as VirtAddr;
        let data_end = &raw const LD_DATA_END as VirtAddr;
        let kernel_start = &raw const LD_KERNEL_START as VirtAddr;

        table
            .map_range(
                text_start,
                text_start - kernel_start + kernel_phys,
                VmFlags::Read | VmFlags::Exec,
                0,
                text_end - text_start,
            )
            .expect("Unable to map the text segment");

        table
            .map_range(
                rodata_start,
                rodata_start - kernel_start + kernel_phys,
                VmFlags::Read,
                0,
                rodata_end - rodata_start,
            )
            .expect("Unable to map the rodata segment");

        table
            .map_range(
                data_start,
                data_start - kernel_start + kernel_phys,
                VmFlags::Read | VmFlags::Write,
                0,
                data_end - data_start,
            )
            .expect("Unable to map the data segment");

        table
            .map_range(
                PageTableEntry::get_hhdm_addr(),
                0,
                VmFlags::Read | VmFlags::Write,
                PageTableEntry::get_hhdm_level(),
                PageTableEntry::get_hhdm_size(),
            )
            .expect("Unable to map identity region");

        // Activate the new page table.
        arch::virt::set_page_table(&table);

        let mut kernel_table = KERNEL_PAGE_TABLE.lock();
        *kernel_table = Some(table);
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

pub fn page_fault_handler(context: &Context) {
    print!("virt: Got a page fault!\n");
    let (mut ip, mut addr) = (0, 0);
    arch::virt::page_fault_handler(context, &mut ip, &mut addr);
    print!("virt: IP: {}, Address: {}\n", ip, addr);
    loop {}
}
