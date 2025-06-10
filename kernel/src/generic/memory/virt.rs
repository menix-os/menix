use super::{
    PhysAddr, VirtAddr,
    pmm::{AllocFlags, PageAllocator},
};
use crate::{
    arch::{self, virt::PageTableEntry},
    generic::{
        process::task::Task,
        util::{align_up, mutex::Mutex},
    },
};
use alloc::alloc::AllocError;
use bitflags::bitflags;
use core::sync::atomic::{AtomicUsize, Ordering};

pub const KERNEL_STACK_SIZE: usize = 0x20000;

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

/// Page caching types.
pub enum VmCacheType {}

#[derive(Debug)]
pub enum PageTableError {
    PageTableEntryMissing,
    LevelOutOfRange,
    NeedAllocation,
}

// TODO: Replace with allocator.
pub static KERNEL_PAGE_TABLE: Mutex<PageTable<true>> = Mutex::new(PageTable::new_kernel_uninit());
pub static KERNEL_MMAP_BASE_ADDR: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum VmLevel {
    L1,
    L2,
    L3,
    #[cfg(target_arch = "riscv64")]
    L4,
    #[cfg(target_arch = "riscv64")]
    L5,
}

/// Represents a virtual address space.
/// `K` controls whether or not this page table is for the kernel or a user process.
#[derive(Debug)]
pub struct PageTable<const K: bool = false> {
    /// Physical address of the root directory.
    head: Mutex<PhysAddr>,
    /// The root page level.
    root_level: usize,
}

impl PageTable<false> {
    /// Creates a new page table for a user process.
    pub fn new_user<P: PageAllocator>(root_level: usize) -> Self {
        Self {
            head: Mutex::new(P::alloc(1, AllocFlags::Zeroed).unwrap()),
            root_level,
        }
    }
}

impl PageTable<true> {
    const fn new_kernel_uninit() -> Self {
        Self {
            head: Mutex::new(PhysAddr(0)),
            root_level: 0,
        }
    }

    pub fn new_kernel<P: PageAllocator>(root_level: usize) -> Self {
        Self {
            head: Mutex::new(P::alloc(1, AllocFlags::Zeroed).unwrap()),
            root_level,
        }
    }

    /// Maps physical memory to a free area in virtual address space.
    pub fn map_memory<P: PageAllocator>(
        &mut self,
        phys: PhysAddr,
        flags: VmFlags,
        level: VmLevel,
        length: usize,
    ) -> Result<*mut u8, AllocError> {
        let aligned_len = align_up(length, arch::virt::get_page_size(VmLevel::L1));

        // Increase mapping base.
        // TODO: Use actual virtual address allocator.
        let virt = KERNEL_MMAP_BASE_ADDR.fetch_add(aligned_len, Ordering::SeqCst);

        // Map memory.
        self.map_range::<P>(VirtAddr(virt), phys, flags, level, aligned_len)
            .map_err(|_| AllocError)?;
        return Ok(virt as *mut u8);
    }
}

impl<const K: bool> PageTable<K> {
    pub const fn root_level(&self) -> usize {
        self.root_level
    }

    /// Sets this page table as the active one.
    ///
    /// # Safety
    ///
    /// All parts of the kernel must still be mapped for this call to be safe.
    pub unsafe fn set_active(&mut self) {
        unsafe {
            arch::virt::set_page_table(*self.head.lock());
        }
    }

    /// Gets the page table entry pointed to by `virt`.
    /// Allocates new levels if necessary and requested.
    /// `target_level`: The level to get for the PTE.
    pub fn get_pte<P: PageAllocator>(
        &self,
        virt: VirtAddr,
        allocate: bool,
        target_level: VmLevel,
    ) -> Result<*mut PageTableEntry, PageTableError> {
        let head = self.head.lock();
        let mut current_head: *mut PageTableEntry = head.as_hhdm();
        let mut index = 0;
        let mut do_break = false;

        // Traverse the page table (from highest to lowest level).
        for level in (0..self.root_level).rev() {
            // Create a mask for the address part of the PTE, e.g. 0x1ff for 9 bits.
            let addr_bits = usize::MAX >> (usize::BITS as usize - arch::virt::get_level_bits());

            // Determine the shift for the appropriate level, e.g. x << (12 + (9 * level)).
            let addr_shift = arch::virt::get_page_bits() + (arch::virt::get_level_bits() * level);

            // Get the index for this level by masking the relevant address part.
            index = (virt.0 >> addr_shift) & addr_bits;

            // The last level is used to access the actual PTE, so break the loop then.
            if level <= target_level as usize || do_break {
                break;
            }

            unsafe {
                let pte = current_head.add(index);

                let mut pte_flags =
                    VmFlags::Directory | if K { VmFlags::None } else { VmFlags::User };

                if (*pte).is_present() {
                    // If this PTE is a large page, it already contains the final address. Don't continue.
                    if !(*pte).is_directory(level) {
                        pte_flags |= VmFlags::Large;

                        // If the caller wanted to go further than this, tell them that it's not possible.
                        if level != target_level as usize {
                            return Err(PageTableError::LevelOutOfRange);
                        }

                        do_break = true;
                    } else {
                        // If the PTE is not large, go one level deeper.
                        current_head = (*pte).address().as_hhdm();
                    }
                    *pte = PageTableEntry::new((*pte).address(), pte_flags, level);
                } else {
                    // PTE isn't present, but we have to allocate a new level now.
                    if !allocate {
                        return Err(PageTableError::NeedAllocation);
                    }

                    // Allocate a new level.
                    let next_head = P::alloc(1, AllocFlags::Zeroed).unwrap().as_hhdm();

                    // ptr::byte_sub() doesn't allow taking higher half addresses because it doesn't fit in an isize.
                    *pte = PageTableEntry::new(
                        VirtAddr::from(next_head)
                            .as_hhdm()
                            .ok_or(PageTableError::PageTableEntryMissing)?,
                        pte_flags,
                        level,
                    );
                    current_head = next_head;
                }
            }
        }

        unsafe {
            return Ok(current_head.add(index));
        }
    }

    /// Establishes a new mapping in this address space.
    /// Fails if the mapping already exists. To overwrite a mapping, use [`Self::remap_single`] instead.
    pub fn map_single<P: PageAllocator>(
        &mut self,
        virt: VirtAddr,
        phys: PhysAddr,
        flags: VmFlags,
        level: VmLevel,
    ) -> Result<(), PageTableError> {
        let pte = self.get_pte::<P>(virt, true, level)?;

        unsafe {
            *pte = PageTableEntry::new(
                phys,
                flags
                    | if level != VmLevel::L1 {
                        VmFlags::Large
                    } else {
                        VmFlags::None
                    },
                level as usize,
            );
        }

        return Ok(());
    }

    /// Changes the permissions on a mapping.
    pub fn remap_single<P: PageAllocator>(
        &mut self,
        virt: VirtAddr,
        flags: VmFlags,
        level: VmLevel,
    ) -> Result<(), PageTableError> {
        let pte = self.get_pte::<P>(virt, false, level)?;

        unsafe {
            *pte = PageTableEntry::new(
                (*pte).address(),
                flags
                    | if level != VmLevel::L1 {
                        VmFlags::Large
                    } else {
                        VmFlags::None
                    },
                level as usize,
            );
        }
        crate::arch::virt::flush_tlb(virt);

        return Ok(());
    }

    /// Maps a range of consecutive memory in this address space.
    pub fn map_range<P: PageAllocator>(
        &mut self,
        virt: VirtAddr,
        phys: PhysAddr,
        flags: VmFlags,
        level: VmLevel,
        length: usize,
    ) -> Result<(), PageTableError> {
        // TODO: Do transactional mapping.
        let length = align_up(length, arch::virt::get_page_size(level));
        let step = arch::virt::get_page_size(level);

        for offset in (0..length).step_by(step) {
            self.map_single::<P>(
                VirtAddr(virt.0 + offset),
                PhysAddr(phys.0 + offset),
                flags,
                level,
            )?;
        }
        return Ok(());
    }

    /// Changes the permissions on a mapping of consecutive memory.
    pub fn remap_range<P: PageAllocator>(
        &mut self,
        virt: VirtAddr,
        flags: VmFlags,
        level: VmLevel,
        length: usize,
    ) -> Result<(), PageTableError> {
        // TODO: Do transactional mapping.
        let length = align_up(length, arch::virt::get_page_size(VmLevel::L1));
        let step = arch::virt::get_page_size(VmLevel::L1)
            + (level as usize * arch::virt::get_level_bits());

        for offset in (0..length).step_by(step) {
            self.remap_single::<P>(VirtAddr(virt.0 + offset), flags, level)?;
        }
        return Ok(());
    }

    /// Un-maps a page from this address space.
    pub fn unmap_single(&mut self, virt: VirtAddr) -> Result<(), PageTableError> {
        crate::arch::virt::flush_tlb(virt);
        todo!();
    }

    /// Un-maps a range from this address space.
    pub fn unmap_range(&mut self, virt: VirtAddr, len: usize) -> Result<(), PageTableError> {
        // TODO
        Ok(())
    }

    /// Checks if the address (may be unaligned) is mapped in this address space.
    pub fn is_mapped(&self, virt: VirtAddr) -> bool {
        let head = self.head.lock();
        let mut current_head: *mut PageTableEntry = head.as_hhdm();
        let mut index;

        for level in (0..self.root_level).rev() {
            let addr_bits = usize::MAX >> (usize::BITS as usize - arch::virt::get_level_bits());
            let addr_shift = arch::virt::get_page_bits() + (arch::virt::get_level_bits() * level);
            index = (virt.0 >> addr_shift) & addr_bits;

            unsafe {
                let pte = current_head.add(index);
                let pte_flags = VmFlags::Directory | if K { VmFlags::None } else { VmFlags::User };

                if (*pte).is_present() {
                    // If this PTE is a large page, it already contains the final address. Don't continue.
                    if !(*pte).is_directory(level) {
                        return true;
                    } else {
                        // If the PTE is not large, go one level deeper.
                        current_head = (*pte).address().as_hhdm();
                        *pte = PageTableEntry::new((*pte).address(), pte_flags, level);
                    }
                } else {
                    return false;
                }
            }
        }

        return false;
    }
}

pub struct AddressSpace {
    table: PageTable,
}

/// Abstract information about a page fault.
pub struct PageFaultInfo {
    /// Fault caused by the user.
    pub caused_by_user: bool,
    /// The instruction pointer address.
    pub ip: VirtAddr,
    /// The address that was attempted to access.
    pub addr: VirtAddr,
    /// The cause of this page fault.
    pub cause: PageFaultCause,
}

bitflags! {
    /// The origin of the page fault.
    #[derive(Debug)]
    pub struct PageFaultCause: usize {
        /// If set, the fault occured in a mapped page.
        const Present = 1 << 0;
        /// If set, the fault was caused by a write.
        const Write = 1 << 1;
        /// If set, the fault was caused by an instruction fetch.
        const Fetch = 1 << 2;
        /// If set, the fault was caused by a user access.
        const User = 1 << 3;
    }
}

/// Generic page fault handler. May reschedule and return a different task to run.
pub fn page_fault_handler(info: &PageFaultInfo) -> *mut Task {
    if info.caused_by_user {
        // TODO: Send SIGSEGV and reschedule.
        // Kill process.
        // Force immediate reschedule.
    }

    panic!(
        "Kernel caused an unrecoverable page fault: {:?}! IP: {:#x}, Address: {:#x}",
        info.cause, info.ip.0, info.addr.0
    );
}

unsafe extern "C" {
    pub unsafe static LD_KERNEL_START: u8;
    pub unsafe static LD_TEXT_START: u8;
    pub unsafe static LD_TEXT_END: u8;
    pub unsafe static LD_RODATA_START: u8;
    pub unsafe static LD_RODATA_END: u8;
    pub unsafe static LD_DATA_START: u8;
    pub unsafe static LD_DATA_END: u8;
}
