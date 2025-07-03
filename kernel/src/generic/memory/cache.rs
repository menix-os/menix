use crate::generic::{
    memory::{
        PhysAddr,
        virt::{PageNumber, VmFlags},
    },
    percpu::CpuData,
    process::{Process, task::Task},
    util::mutex::Mutex,
};
use alloc::{
    collections::btree_map::BTreeMap,
    sync::{Arc, Weak},
};
use core::fmt::Debug;

/// A virtual memory object. It represents a single mapped entity and its backing storage.
#[derive(Debug)]
pub struct Object {
    pub backing: Option<Arc<Object>>,
    /// All currently present pages of this object. Keys are offsets (in pages) to physical pages.
    pub present_pages: Mutex<BTreeMap<PageNumber, PhysAddr>>,
    /// Used to read and write back pages from/to a backing storage.
    pub pager: Weak<dyn Pager>,
    /// The protection flags for this mapping.
    pub protection: VmFlags,
}

impl Object {
    /// Creates a new anonymous object.
    pub fn new_anon(num_pages: usize, protection: VmFlags) -> Self {
        let anon: Arc<dyn Pager> = Arc::new(AnonPager {});
        Self {
            backing: None,
            present_pages: Mutex::new(BTreeMap::new()),
            pager: Arc::downgrade(&anon),
            protection,
        }
    }

    pub fn new_paged(num_pages: usize, protection: VmFlags, pager: Weak<dyn Pager>) -> Self {
        Self {
            backing: None,
            present_pages: Mutex::new(BTreeMap::new()),
            pager,
            protection,
        }
    }
}

pub enum PagerError {}
pub trait Pager: Debug {
    /// Reads pages from the backing storage. `pages` is an array of page numbers to read
    fn get_pages(
        &self,
        object: &Object,
        pages: &[PageNumber],
        faulty_page: PageNumber,
    ) -> Result<&[PageNumber], PagerError>;

    /// Writes pages to the backing store.
    fn write_pages(&self, object: &Object, pages: &[PageNumber]) -> Result<(), PagerError>;
}

#[derive(Debug)]
struct AnonPager {}
impl Pager for AnonPager {
    fn get_pages(
        &self,
        object: &Object,
        pages: &[PageNumber],
        faulty_page: PageNumber,
    ) -> Result<&[PageNumber], PagerError> {
        todo!()
    }

    fn write_pages(&self, object: &Object, pages: &[PageNumber]) -> Result<(), PagerError> {
        todo!()
    }
}

init_stage! {
    #[depends(crate::generic::process::sched::SCHEDULER_STAGE)]
    CACHE_STAGE: "generic.memory.cache" => init;
}

fn init() {
    let task = Task::new(cache_flush, 0, 0, Process::get_kernel(), false)
        .expect("Unable to start page cache task");
    CpuData::get().scheduler.add_task(Arc::new(task));
}

extern "C" fn cache_flush(_: usize, _: usize) {
    // TODO
}
