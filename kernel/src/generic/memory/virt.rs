use super::{
    PhysAddr, VirtAddr,
    pmm::{AllocFlags, PageAllocator},
};
use crate::{
    arch::{self, virt::PageTableEntry},
    generic::{
        memory::{cache::MemoryObject, pmm::KernelAlloc},
        posix::errno::{EResult, Errno},
        sched::Scheduler,
        util::{align_up, divide_up, once::Once, spin_mutex::SpinMutex},
    },
};
use alloc::{alloc::AllocError, collections::btree_map::BTreeMap, slice, sync::Arc};
use bitflags::bitflags;
use core::{
    num::NonZeroUsize,
    sync::atomic::{AtomicUsize, Ordering},
};

// TODO: Kernel stacks should be mapped, not just on the HHDM. Otherwise we can't check for overflows.
pub const KERNEL_STACK_SIZE: usize = 0x8000;

bitflags! {
    /// Page protection flags.
    #[derive(Debug, Copy, Clone)]
    pub struct VmFlags: u8 {
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

pub(crate) static KERNEL_PAGE_TABLE: Once<Arc<PageTable>> = Once::new();

// TODO: Replace with allocator.
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
#[derive(Debug)]
pub struct PageTable {
    /// Physical address of the root directory.
    head: SpinMutex<PhysAddr>,
    /// The root page level.
    root_level: usize,
    /// `true`, if this is a user page table.
    is_user: bool,
}

impl PageTable {
    /// Creates a new page table for a user process.
    pub fn new_user<P: PageAllocator>(flags: AllocFlags) -> Self {
        // We need to have the higher half mapped in every user map for this to work.
        let user_l1 = P::alloc(1, flags | AllocFlags::Zeroed).unwrap();
        unsafe {
            let user_l1_slice: &mut [u8] = slice::from_raw_parts_mut(
                user_l1.as_hhdm(),
                arch::virt::get_page_size(VmLevel::L1),
            );
            let kernel_l1_slice: &mut [u8] = slice::from_raw_parts_mut(
                KERNEL_PAGE_TABLE.get().head.lock().as_hhdm(),
                arch::virt::get_page_size(VmLevel::L1),
            );
            user_l1_slice.copy_from_slice(&kernel_l1_slice);
        }
        Self {
            head: SpinMutex::new(user_l1),
            root_level: KERNEL_PAGE_TABLE.get().root_level,
            is_user: true,
        }
    }

    /// Creates a new page table for a kernel process.
    pub fn new_kernel<P: PageAllocator>(root_level: usize, flags: AllocFlags) -> Self {
        Self {
            head: SpinMutex::new(P::alloc(1, flags | AllocFlags::Zeroed).unwrap()),
            root_level,
            is_user: false,
        }
    }

    pub fn get_kernel() -> &'static PageTable {
        KERNEL_PAGE_TABLE.get()
    }

    /// Maps physical memory to a free area in virtual address space.
    pub fn map_memory<P: PageAllocator>(
        &self,
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

impl PageTable {
    /// Returns the physical address of the top level.
    pub fn get_head_addr(&self) -> PhysAddr {
        *self.head.lock()
    }

    pub const fn root_level(&self) -> usize {
        self.root_level
    }

    /// Sets this page table as the active one.
    ///
    /// # Safety
    ///
    /// All parts of the kernel must still be mapped for this call to be safe.
    pub unsafe fn set_active(&self) {
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

                let mut pte_flags = VmFlags::Directory
                    | if self.is_user {
                        VmFlags::User
                    } else {
                        VmFlags::empty()
                    };

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

    /// Establishes a new mapping in this page table.
    /// Fails if the mapping already exists. To overwrite a mapping, use [`Self::remap_single`] instead.
    pub fn map_single<P: PageAllocator>(
        &self,
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
                    | if self.is_user {
                        VmFlags::User
                    } else {
                        VmFlags::empty()
                    }
                    | if level != VmLevel::L1 {
                        VmFlags::Large
                    } else {
                        VmFlags::empty()
                    },
                level as usize,
            );
        }

        return Ok(());
    }

    /// Changes the permissions on a mapping.
    pub fn remap_single<P: PageAllocator>(
        &self,
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
                        VmFlags::empty()
                    },
                level as usize,
            );
        }
        crate::arch::virt::flush_tlb(virt);

        return Ok(());
    }

    /// Maps a range of consecutive memory in this page table.
    pub fn map_range<P: PageAllocator>(
        &self,
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
        &self,
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

    /// Un-maps a page from this page table.
    pub fn unmap_single(&self, virt: VirtAddr) -> Result<(), PageTableError> {
        crate::arch::virt::flush_tlb(virt);

        // TODO
        Ok(())
    }

    /// Un-maps a range from this page table.
    pub fn unmap_range(&self, virt: VirtAddr, len: usize) -> Result<(), PageTableError> {
        // TODO
        Ok(())
    }

    /// Checks if the address (may be unaligned) is mapped in this page table.
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
                let pte_flags = VmFlags::Directory
                    | if self.is_user {
                        VmFlags::User
                    } else {
                        VmFlags::empty()
                    };

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

#[derive(Debug)]
pub struct AddressSpace {
    pub table: Arc<PageTable>,
    /// A map that translates global page offsets (virt / page_size) to a physical page and the flags of the mapping.
    pub mappings: SpinMutex<BTreeMap<usize, MappedObject>>,
}

impl Clone for AddressSpace {
    fn clone(&self) -> Self {
        let maps = self.mappings.lock().clone();
        Self {
            table: Arc::new(if self.table.is_user {
                PageTable::new_user::<KernelAlloc>(AllocFlags::empty())
            } else {
                PageTable::new_kernel::<KernelAlloc>(self.table.root_level, AllocFlags::empty())
            }),
            mappings: SpinMutex::new(maps),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MappedObject {
    offset_page: usize,
    object: Arc<MemoryObject>,
    flags: VmFlags,
}

impl AddressSpace {
    pub fn new() -> Self {
        Self {
            table: Arc::new(PageTable::new_user::<KernelAlloc>(AllocFlags::empty())),
            mappings: SpinMutex::default(),
        }
    }

    /// Maps an object into the address space.
    pub fn map_object(
        &self,
        object: Arc<MemoryObject>,
        addr: VirtAddr,
        len: NonZeroUsize,
        prot: VmFlags,
        offset: uapi::off_t,
    ) -> EResult<()> {
        // `addr + len` may not overflow if the mapping is fixed.
        if addr.value().checked_add(len.into()).is_none() {
            return Err(Errno::ENOMEM);
        }

        let page_size = arch::virt::get_page_size(VmLevel::L1);
        if addr.value() % page_size != offset as usize % page_size {
            return Err(Errno::EINVAL);
        }

        let mut mappings = self.mappings.lock();

        // We need enough pages to fit all bytes.
        let num_pages = divide_up(len.into(), page_size);
        let start_page = addr.value() / page_size;
        let offset_page = offset as usize / page_size;

        for p in 0..num_pages {
            _ = object.try_get_page(p + offset_page).ok_or(Errno::EINVAL)?;
            // Create a mapping for this address space.
            let mapped = MappedObject {
                offset_page: p + offset_page,
                object: object.clone(),
                flags: prot,
            };
            mappings.insert(start_page + p, mapped);
        }

        Ok(())
    }

    pub fn protect(&self, addr: VirtAddr, len: NonZeroUsize, prot: VmFlags) -> EResult<()> {
        // `addr + len` may not overflow if the mapping is fixed.
        if addr.value().checked_add(len.into()).is_none() {
            return Err(Errno::ENOMEM);
        }

        let page_size = arch::virt::get_page_size(VmLevel::L1);
        if addr.value() % page_size != 0 {
            return Err(Errno::EINVAL);
        }

        let num_pages = divide_up(len.into(), page_size);
        let start_page = addr.value() / page_size;
        let mut mappings = self.mappings.lock();

        for p in 0..num_pages {
            let mapped = mappings.get_mut(&(start_page + p)).ok_or(Errno::EINVAL)?;
            mapped.flags = prot;
        }

        Ok(())
    }

    pub fn clear(&self) {
        self.mappings.lock().clear();
    }
}

/// Abstract information about a page fault.
pub struct PageFaultInfo {
    /// The instruction pointer address at the point of the page fault.
    pub ip: VirtAddr,
    /// The address that was attempted to access.
    pub addr: VirtAddr,
    /// If set, the fault was caused by a user access.
    pub caused_by_user: bool,
    /// If set, the fault was caused by a write.
    pub caused_by_write: bool,
    /// If set, the fault was caused by an instruction fetch.
    pub caused_by_fetch: bool,
    /// If set, the fault occured in a present page.
    pub page_was_present: bool,
}

/// Generic page fault handler. May reschedule and return a different task to run.
pub fn page_fault_handler(info: &PageFaultInfo) {
    // Check if the current address space has a theoretical mapping at the faulting address.
    let proc = Scheduler::get_current().get_process();
    let inner = proc.inner.lock();
    let space = &inner.address_space;
    // The page index of the page fault address.
    let faulty_page = info.addr.value() / arch::virt::get_page_size(VmLevel::L1);
    if let Some(mapped) = inner.address_space.mappings.lock().get(&faulty_page) {
        if let Some(phys) = mapped.object.try_get_page(mapped.offset_page) {
            // If we get here, the accessed address is valid. Map it in the actual page table and return.
            space
                .table
                .map_single::<KernelAlloc>(info.addr, phys, mapped.flags, VmLevel::L1)
                .expect("Failed to map a demand-loaded page");
            return;
        }
    }

    if info.caused_by_user {
        // TODO: Send SIGSEGV and reschedule.
        // Kill process.
        // Force immediate reschedule.
        panic!("User process caused a segmentation fault!");
    }

    // If any other attempt to recover has failed, we made a mistake.
    panic!(
        "Kernel caused an unrecoverable page fault. Attempted to {} a {} page at {:#x} (IP: {:#x})",
        if info.caused_by_write {
            "write to"
        } else if info.caused_by_fetch {
            "execute on"
        } else {
            "read from"
        },
        if info.page_was_present {
            "present"
        } else {
            "non-present"
        },
        info.addr.0,
        info.ip.0
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
