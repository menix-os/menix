use crate::generic::{
    percpu::CpuData,
    process::{Process, task::Task},
};
use alloc::sync::Arc;

/// A cached memory object.
#[derive(Debug)]
pub struct Object {
    backing: Option<Arc<Object>>,
}

impl Object {
    pub fn new() -> Self {
        // TODO
        Self { backing: None }
    }
}

pub enum helper {}

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
