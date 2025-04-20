use super::{PageAlloc, PhysAddr, VirtAddr, phys};
use crate::arch::{self, page::PageTableEntry};
use alloc::vec::Vec;
use alloc::{boxed::Box, sync::Arc};
use bitflags::bitflags;
use core::{marker::PhantomData, ops::Deref};
use spin::{Mutex, RwLock};

// User constants
pub const USER_STACK_SIZE: usize = 0x200000;
pub const USER_STACK_BASE: usize = 0x00007F0000000000;
pub const USER_MAP_BASE: usize = 0x0000600000000000;

// Kernel constants
pub const KERNEL_STACK_SIZE: usize = 0x20000;
pub const MAP_BASE: usize = 0xFFFF90000000000;
pub const MEMORY_BASE: usize = 0xFFFFA0000000000;
pub const MODULE_BASE: usize = 0xFFFFB0000000000;

const PAGE_TABLE_SIZE: usize = (PageTableEntry::get_page_size()) / size_of::<PageTableEntry>();

bitflags! {
    /// Page protection flags.
    #[derive(Debug, Copy, Clone)]
    pub struct VmFlags: usize {
        const None = 0;
        /// Page can be read from.
        const Read = 1 << 0;
        /// Page can be written to.
        const Write = 1 << 1;
        /// Page has executable code.
        const Exec = 1 << 2;
        /// Page can be accessed by the user.
        const User = 1 << 3;
        /// Page is a large page.
        const Large = 1 << 4;
        /// Page is a directory to the next level.
        const Directory = 1 << 5;
    }
}

#[derive(Debug)]
pub enum PageTableError {
    PageTableEntryMissing,
    LevelOutOfRange,
    NeedAllocation,
}

pub static KERNEL_PAGE_TABLE: RwLock<PageTable<true>> = RwLock::new(PageTable::new_kernel());

/// Represents a virtual address space.
/// `K` controls whether or not this page table is for the kernel or a user process.
pub struct PageTable<const K: bool = false> {
    pub head: Mutex<Option<Box<[PageTableEntry; PAGE_TABLE_SIZE], PageAlloc>>>,
}

impl PageTable<false> {
    /// Creates a new page table for a user process.
    pub const fn new_user() -> Self {
        return Self {
            head: Mutex::new(None),
        };
    }
}

impl PageTable<true> {
    const fn new_kernel() -> Self {
        return Self {
            head: Mutex::new(None),
        };
    }

    /// Maps physical memory to a free area in virtual address space.
    pub fn map_memory(
        &mut self,
        phys: PhysAddr,
        flags: VmFlags,
        level: usize,
        length: usize,
    ) -> *mut u8 {
        // TODO: Get next free memory region.
        return (PageTableEntry::get_hhdm_addr().0 + phys.0) as *mut u8;
    }
}

impl<const K: bool> PageTable<K> {
    /// Sets this page table as the active one.
    ///
    /// # Safety
    ///
    /// All parts of the kernel must still be mapped for this call to be safe.
    pub unsafe fn set_active(&mut self) {
        unsafe {
            let virt = self
                .head
                .lock()
                .as_mut()
                .expect("Page table should contain at least the root level")
                .as_ptr();
            let addr = PhysAddr(virt as usize);
            arch::page::set_page_table(addr);
        }
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
    ) -> Result<&mut PageTableEntry, PageTableError> {
        let mut head = self.head.lock();

        // If there is no head yet, allocate it.
        let mut head = match &mut *head {
            Some(x) => x,
            None => &mut Box::new_in([PageTableEntry::empty(); PAGE_TABLE_SIZE], PageAlloc),
        };

        let mut current_head = head.as_mut_ptr();
        let mut index = 0;
        let mut do_break = false;

        if target_level >= PageTableEntry::get_levels() - 1 {
            return Err(PageTableError::LevelOutOfRange);
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
            index = (virt.0 >> addr_shift) & addr_bits;

            // The last level is used to access the actual PTE, so break the loop then.
            if level <= target_level || do_break {
                break;
            }

            unsafe {
                let pte = current_head.add(index);

                let mut pte_flags =
                    VmFlags::Directory | if K { VmFlags::None } else { VmFlags::User };

                if (*pte).is_present() {
                    // If this PTE is a large page, it already contains the final address. Don't continue.
                    if !(*pte).is_directory() {
                        pte_flags |= VmFlags::Large;

                        // If the caller wanted to go further than this, tell them that it's not possible.
                        if level != target_level {
                            return Err(PageTableError::LevelOutOfRange);
                        }

                        do_break = true;
                    } else {
                        // If the PTE is not large, go one level deeper.
                        current_head = ((*pte).address().0 + PageTableEntry::get_hhdm_addr().0)
                            as *mut PageTableEntry;
                    }
                    *pte = PageTableEntry::new((*pte).address(), pte_flags);
                } else {
                    // PTE isn't present, we have to allocate a new level.
                    if !allocate {
                        return Err(PageTableError::NeedAllocation);
                    }

                    let next_head = Box::leak(Box::new_in(
                        [PageTableEntry::empty(); PAGE_TABLE_SIZE],
                        PageAlloc,
                    ))
                    .as_mut_ptr();
                    // ptr::byte_sub() doesn't allow taking higher half addresses because it doesn't fit in an isize.
                    *pte = PageTableEntry::new(
                        VirtAddr::from(next_head)
                            .get_kernel_phys()
                            .ok_or(PageTableError::PageTableEntryMissing)?,
                        pte_flags,
                    );
                    current_head = next_head;
                }
            }
        }

        unsafe {
            return current_head
                .add(index)
                .as_mut()
                .ok_or(PageTableError::PageTableEntryMissing);
        }
    }

    /// Establishes a new mapping in this address space.
    /// Fails if the mapping already exists. To overwrite a mapping, use [`Self::remap_single`] instead.
    pub fn map_single(
        &mut self,
        virt: VirtAddr,
        phys: PhysAddr,
        flags: VmFlags,
        level: usize,
    ) -> Result<(), PageTableError> {
        let pte = self.get_pte(virt, true, level)?;

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

    pub fn remap_single() -> Result<(), PageTableError> {
        todo!();
    }

    /// Maps a range of consecutive memory in this address space.
    pub fn map_range(
        &mut self,
        virt: VirtAddr,
        phys: PhysAddr,
        flags: VmFlags,
        level: usize,
        length: usize,
    ) -> Result<(), PageTableError> {
        let step =
            1 << (PageTableEntry::get_page_bits() + (level * PageTableEntry::get_level_bits()));

        for offset in (0..length).step_by(step) {
            self.map_single(
                VirtAddr(virt.0 + offset),
                PhysAddr(phys.0 + offset),
                flags,
                level,
            )?;
        }
        return Ok(());
    }

    /// Un-maps a page from this address space.
    pub fn unmap(&mut self, virt: VirtAddr) -> Result<(), PageTableError> {
        Ok(())
    }

    /// Un-maps a range from this address space.
    pub fn unmap_range(&mut self, virt: VirtAddr, len: usize) -> Result<(), PageTableError> {
        Ok(())
    }

    /// Checks if the address (may be unaligned) is mapped in this address space.
    pub fn is_mapped(&self, virt: VirtAddr, level: usize) -> bool {
        let pte = self.get_pte(virt, false, level);
        match pte {
            Ok(x) => x.is_present(),
            Err(_) => {
                return false;
            }
        }
    }
}

/// Initialize the kernel page table and switch to it.
#[deny(dead_code)]
pub fn init(temp_hhdm: VirtAddr, kernel_phys: PhysAddr, kernel_virt: VirtAddr) {
    let mut table = PageTable::new_kernel();

    unsafe {
        let text_start = VirtAddr(&raw const LD_TEXT_START as usize);
        let text_end = VirtAddr(&raw const LD_TEXT_END as usize);
        let rodata_start = VirtAddr(&raw const LD_RODATA_START as usize);
        let rodata_end = VirtAddr(&raw const LD_RODATA_END as usize);
        let data_start = VirtAddr(&raw const LD_DATA_START as usize);
        let data_end = VirtAddr(&raw const LD_DATA_END as usize);
        let kernel_start = VirtAddr(&raw const LD_KERNEL_START as usize);

        table
            .map_range(
                text_start,
                PhysAddr(text_start.0 - kernel_start.0 + kernel_phys.0),
                VmFlags::Read | VmFlags::Exec,
                0,
                text_end.0 - text_start.0,
            )
            .expect("Unable to map the text segment");

        table
            .map_range(
                rodata_start,
                PhysAddr(rodata_start.0 - kernel_start.0 + kernel_phys.0),
                VmFlags::Read,
                0,
                rodata_end.0 - rodata_start.0,
            )
            .expect("Unable to map the rodata segment");

        table
            .map_range(
                data_start,
                PhysAddr(data_start.0 - kernel_start.0 + kernel_phys.0),
                VmFlags::Read | VmFlags::Write,
                0,
                data_end.0 - data_start.0,
            )
            .expect("Unable to map the data segment");

        table
            .map_range(
                PageTableEntry::get_hhdm_addr(),
                PhysAddr(0),
                VmFlags::Read | VmFlags::Write,
                PageTableEntry::get_hhdm_level(),
                PageTableEntry::get_hhdm_size(),
            )
            .expect("Unable to map identity region");

        // Activate the new page table.
        table.set_active();

        let mut kernel_table = KERNEL_PAGE_TABLE.write();
        *kernel_table = table;
    }
}

/// Wraps a *T from a different address space.
pub struct ForeignPtr<T> {
    page_table: Arc<PageTable>,
    addr: VirtAddr,
    _p: PhantomData<T>,
}

impl<T> ForeignPtr<T> {
    pub const fn new(page_table: Arc<PageTable>, addr: VirtAddr) -> Self {
        Self {
            page_table,
            addr,
            _p: PhantomData,
        }
    }

    /// Gets the numeric value of this pointer.
    pub const fn value(&self) -> VirtAddr {
        return self.addr;
    }
}

impl<T> Deref for ForeignPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
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
