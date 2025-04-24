use super::{HHDM_START, PhysAddr, VirtAddr, phys};
use crate::arch::irq::InterruptFrame;
use crate::arch::{self, virt::PageTableEntry};
use crate::generic::boot::BootInfo;
use crate::generic::misc::align_up;
use crate::generic::{self, misc};
use alloc::alloc::AllocError;
use alloc::vec::Vec;
use alloc::{boxed::Box, sync::Arc};
use bitflags::bitflags;
use core::num::NonZero;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::{marker::PhantomData, ops::Deref};
use spin::{Mutex, RwLock};

// User constants
pub const USER_STACK_SIZE: usize = 0x200000;
pub const USER_STACK_BASE: usize = 0x00007F0000000000;
pub const USER_MAP_BASE: usize = 0x0000600000000000;

// Kernel constants
pub const KERNEL_STACK_SIZE: usize = 0x20000;

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

/// Page caching types.
pub enum VmCacheType {}

#[derive(Debug)]
pub enum PageTableError {
    PageTableEntryMissing,
    LevelOutOfRange,
    NeedAllocation,
}

pub static KERNEL_PAGE_TABLE: RwLock<PageTable<true>> = RwLock::new(PageTable::new_kernel_uninit());
pub static KERNEL_MMAP_BASE_ADDR: AtomicUsize = AtomicUsize::new(0);

/// Represents a virtual address space.
/// `K` controls whether or not this page table is for the kernel or a user process.
pub struct PageTable<const K: bool = false> {
    /// Physical address of the root directory.
    head: Mutex<PhysAddr>,
    /// The highest supported level for mappings.
    max_level: usize,
}

impl PageTable<false> {
    /// Creates a new page table for a user process.
    pub fn new_user(max_level: usize) -> Self {
        Self {
            head: Mutex::new(
                phys::alloc_bytes(
                    NonZero::new(PageTableEntry::get_page_size()).unwrap(),
                    phys::RegionType::Kernel,
                )
                .unwrap(),
            ),
            max_level,
        }
    }
}

impl PageTable<true> {
    const fn new_kernel_uninit() -> Self {
        Self {
            head: Mutex::new(PhysAddr(0)),
            max_level: 0,
        }
    }

    fn new_kernel(max_level: usize) -> Self {
        Self {
            head: Mutex::new(
                phys::alloc_bytes(
                    NonZero::new(PageTableEntry::get_page_size()).unwrap(),
                    phys::RegionType::Kernel,
                )
                .unwrap(),
            ),
            max_level,
        }
    }

    /// Maps physical memory to a free area in virtual address space.
    pub fn map_memory(
        &mut self,
        phys: PhysAddr,
        flags: VmFlags,
        level: usize,
        length: usize,
    ) -> Result<*mut u8, AllocError> {
        let aligned_len = misc::align_up(length, PageTableEntry::get_page_size());

        // Increase mapping base.
        // TODO: Use actual virtual address allocator.
        let virt = KERNEL_MMAP_BASE_ADDR.fetch_add(aligned_len, Ordering::SeqCst);

        // Map memory.
        self.map_range(VirtAddr(virt), phys, flags, level, aligned_len);
        return Ok(virt as *mut u8);
    }
}

impl<const K: bool> PageTable<K> {
    /// Sets this page table as the active one.
    ///
    /// # Safety
    ///
    /// All parts of the kernel must still be mapped for this call to be safe.
    pub unsafe fn set_active(&mut self) {
        let addr = self.head.lock();
        unsafe {
            arch::virt::set_page_table(*addr);
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
        let mut current_head: *mut PageTableEntry = head.as_hhdm();
        let mut index = 0;
        let mut do_break = false;

        if target_level > self.max_level {
            return Err(PageTableError::LevelOutOfRange);
        }

        // Traverse the page table (from highest to lowest level).
        for level in (0..self.max_level).rev() {
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
                        current_head = (*pte).address().as_hhdm();
                    }
                    *pte = PageTableEntry::new((*pte).address(), pte_flags);
                } else {
                    // PTE isn't present, but we have to allocate a new level now.
                    if !allocate {
                        return Err(PageTableError::NeedAllocation);
                    }

                    // Allocate a new level.
                    let next_head = phys::alloc_bytes(
                        NonZero::new(PageTableEntry::get_page_size()).unwrap(),
                        phys::RegionType::Kernel,
                    )
                    .unwrap()
                    .as_hhdm();

                    // ptr::byte_sub() doesn't allow taking higher half addresses because it doesn't fit in an isize.
                    *pte = PageTableEntry::new(
                        VirtAddr::from(next_head)
                            .as_hhdm()
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
        // TODO: Do transactional mapping.
        let length = align_up(length, PageTableEntry::get_page_size());
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
        todo!();
    }

    /// Un-maps a range from this address space.
    pub fn unmap_range(&mut self, virt: VirtAddr, len: usize) -> Result<(), PageTableError> {
        todo!();
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

/// Initialize the kernel's own page table and switch to it.
#[deny(dead_code)]
pub fn init(
    hhdm_start: VirtAddr,
    hhdm_length: usize,
    paging_level: usize,
    kernel_phys: PhysAddr,
    kernel_virt: VirtAddr,
) {
    let mut table = PageTable::new_kernel(paging_level);

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
        print!("virt: Loaded text segment at {:#018X}.\n", text_start.0);

        table
            .map_range(
                rodata_start,
                PhysAddr(rodata_start.0 - kernel_start.0 + kernel_phys.0),
                VmFlags::Read,
                0,
                rodata_end.0 - rodata_start.0,
            )
            .expect("Unable to map the rodata segment");
        print!("virt: Loaded rodata segment at {:#018X}.\n", rodata_start.0);

        table
            .map_range(
                data_start,
                PhysAddr(data_start.0 - kernel_start.0 + kernel_phys.0),
                VmFlags::Read | VmFlags::Write,
                0,
                data_end.0 - data_start.0,
            )
            .expect("Unable to map the data segment");
        print!("virt: Loaded data segment at {:#018X}.\n", data_start.0);

        table
            .map_range(
                hhdm_start,
                PhysAddr(0),
                VmFlags::Read | VmFlags::Write,
                2,
                hhdm_length,
            )
            .expect("Unable to map HHDM region");
        print!("virt: Loaded HHDM segment at {:#018X}.\n", hhdm_start.0);

        print!("virt: Installing kernel page table...\n");

        // Activate the new page table.
        table.set_active();

        print!("virt: Kernel map is now active\n");

        // Save the page table.
        let mut kernel_table = KERNEL_PAGE_TABLE.write();
        *kernel_table = table;

        // Set the MMAP base to right after the HHDM. Make sure this lands on a new PTE so we can map regular pages.
        let offset = misc::align_up(
            hhdm_length,
            1usize
                << (paging_level * PageTableEntry::get_level_bits()
                    + PageTableEntry::get_level_bits()
                    + 1),
        );
        KERNEL_MMAP_BASE_ADDR.store(hhdm_start.0 + offset, Ordering::Relaxed);
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

/// Abstract information about a page fault.
pub struct PageFaultInfo {
    /// Fault caused by the user.
    pub caused_by_user: bool,
    /// The instruction pointer address.
    pub ip: VirtAddr,
    /// The address that was attempted to access.
    pub addr: VirtAddr,
    /// The cause of this page fault.
    pub kind: PageFaultKind,
}

/// The origin of the page fault.
pub enum PageFaultKind {
    /// Issue unclear (possible corruption).
    Unknown,
    /// Page is not mapped in the current page table.
    NotMapped,
    /// Page is mapped, but can't be read from.
    IllegalRead,
    /// Page is mapped, but can't be written to.
    IllegalWrite,
    /// Page is mapped, but can't be executed on.
    IllegalExecute,
}

/// Generic page fault handler. May reschedule and return a different context.
pub fn page_fault_handler<'a>(
    context: &'a InterruptFrame,
    info: &PageFaultInfo,
) -> &'a InterruptFrame {
    if info.caused_by_user {
        // TODO: Send SIGSEGV and reschedule.
        return context;
    }

    panic!(
        "Kernel caused an unrecoverable page fault: {}! IP: {:#x}, Address: {:#x}",
        match info.kind {
            PageFaultKind::Unknown => "Unknown",
            PageFaultKind::NotMapped => "Page was not mapped",
            PageFaultKind::IllegalRead => "Page can't be read from",
            PageFaultKind::IllegalWrite => "Page can't be written to",
            PageFaultKind::IllegalExecute => "Page can't be executed on",
        },
        info.ip.0,
        info.addr.0
    );
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
