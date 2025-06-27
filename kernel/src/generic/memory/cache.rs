use crate::{
    arch,
    generic::{
        clock,
        device::BlockDevice,
        memory::{
            PhysAddr,
            pmm::{AllocFlags, KernelAlloc, Page, PageAllocator},
            virt::VmLevel,
        },
        percpu::CpuData,
        posix::errno::EResult,
        process::{Process, task::Task},
        util::{align_up, mutex::Mutex},
        vfs::inode::INode,
    },
};
use alloc::{collections::btree_map::BTreeMap, sync::Arc};
use bytemuck::AnyBitPattern;
use core::sync::atomic::Ordering;

#[derive(Debug, Default)]
pub struct PageCache {
    cache: Mutex<BTreeMap<u64, RcPage>>,
}

impl PageCache {
    pub fn get_data_at(&self, offset: u64) -> Option<RcPage> {
        self.cache.lock().get(&offset).cloned()
    }

    pub fn sync(&self) -> EResult<()> {
        let time = clock::get_elapsed();
        let cache = self.cache.lock();
        for (_, page) in cache.iter() {
            page.write_back(Some(time))?;
        }

        Ok(())
    }
}

/// A delay of 100ms.
const WRITE_BACK_DELAY_NS: usize = 100_000_000;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct RcPage(Arc<InnerPagedData>);

impl RcPage {
    /// Allocates a new buffer to fit at least `bytes` amount of bytes.
    pub fn new(
        bytes: usize,
        flags: AllocFlags,
        device_offset: u64,
        backing: BackingKind,
    ) -> EResult<Self> {
        let num_pages = align_up(bytes, arch::virt::get_page_size(VmLevel::L1))
            / arch::virt::get_page_size(VmLevel::L1);

        Ok(Self(Arc::new(InnerPagedData {
            start: KernelAlloc::alloc(num_pages, flags)?,
            num_pages,
            device_offset,
            backing,
        })))
    }

    /// Writes the pages back to its backing storage.
    pub fn write_back(&self, timestamp: Option<usize>) -> EResult<()> {
        let db = super::pmm::PAGE_DB.lock();
        let start = Page::idx_from_addr(self.0.start);

        // Iterate over all pages and write back if necessary.
        for page_idx in 0..self.0.num_pages {
            let page = &db[start + page_idx];

            // If we've already written
            if let Some(t) = timestamp {
                let last_write = page.last_write.load(Ordering::Acquire);
                if t < last_write + WRITE_BACK_DELAY_NS {
                    continue;
                }
            }

            // Ignore pages that weren't dirty to begin with.
            // If they were dirty, mark them as clean now.
            if !page.dirty.swap(false, Ordering::Acquire) {
                continue;
            }

            // Write back the page to the backing storage.
            match &self.0.backing {
                BackingKind::Anonymous => {} // Nothing to do for anonymous storage, since it lives in memory.
                BackingKind::BlockDevice(b) => {
                    b.ops.write_data(self.0.device_offset, self.as_slice())?
                }
                BackingKind::FileSystem(inode) => {
                    todo!()
                }
            }

            if let Some(t) = timestamp {
                page.last_write.store(t, Ordering::Release);
            }
        }

        Ok(())
    }

    /// Returns an immutable slice pointing to the page data.
    pub fn as_slice<T: AnyBitPattern>(&self) -> &[T] {
        unsafe {
            core::slice::from_raw_parts(
                self.0.start.as_hhdm() as *const T,
                self.0.num_pages * arch::virt::get_page_size(VmLevel::L1),
            )
        }
    }

    /// Returns a mutable slice pointing to the page data.
    /// # Safety
    /// The caller must ensure that there is only one simultaneous write access on this data.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_slice_mut<T: AnyBitPattern>(&self) -> &mut [T] {
        unsafe {
            core::slice::from_raw_parts_mut(
                self.0.start.as_hhdm() as *mut T,
                self.0.num_pages * arch::virt::get_page_size(VmLevel::L1),
            )
        }
    }
}

#[derive(Debug)]
struct InnerPagedData {
    /// Start address of the data.
    start: PhysAddr,
    /// Length of the allocation in pages.
    num_pages: usize,
    /// Offset into the device file.
    device_offset: u64,
    /// The kind of storage which this cache is for.
    backing: BackingKind,
}

impl Drop for InnerPagedData {
    fn drop(&mut self) {
        unsafe { KernelAlloc::dealloc(self.start, self.num_pages) };
    }
}

#[derive(Debug, Clone)]
pub enum BackingKind {
    /// No backing storage, anonymous mapping.
    Anonymous,
    /// Backed by a block device.
    BlockDevice(Arc<BlockDevice>),
    /// Backed by an inode.
    FileSystem(Arc<INode>),
}

impl BackingKind {
    pub fn get_cache(&self) -> Option<&PageCache> {
        match self {
            BackingKind::Anonymous => None,
            BackingKind::BlockDevice(b) => Some(&b.cache),
            BackingKind::FileSystem(n) => Some(&n.cache),
        }
    }
}

init_stage! {
    #[depends(crate::generic::process::sched::SCHEDULER_STAGE)]
    PAGE_CACHE_STAGE: "generic.memory.page-cache" => init;
}

fn init() {
    let task = Task::new(cache_flush, 0, 0, Process::get_kernel(), false)
        .expect("Unable to start page cache task");
    CpuData::get().scheduler.add_task(Arc::new(task));
}

extern "C" fn cache_flush(_: usize, _: usize) {
    // TODO
}
