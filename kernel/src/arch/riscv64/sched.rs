use crate::generic::{memory::VirtAddr, posix::errno::EResult, process::task::Task};

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Context {
    pub ra: u64,
    pub sp: u64,
    pub gp: u64,
    pub tp: u64,
    pub t0: u64,
    pub t1: u64,
    pub t2: u64,
    pub s0: u64,
    pub s1: u64,
    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
    pub a3: u64,
    pub a4: u64,
    pub a5: u64,
    pub a6: u64,
    pub a7: u64,
    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub s8: u64,
    pub s9: u64,
    pub s10: u64,
    pub s11: u64,
    pub t3: u64,
    pub t4: u64,
    pub t5: u64,
    pub t6: u64,
}

impl Context {
    pub fn set_return(&mut self, val: usize, err: usize) {
        self.a0 = val as _;
        self.a1 = err as _;
    }
}

#[derive(Debug, Default, Clone)]
pub struct TaskContext {}

pub fn get_task() -> *const Task {
    todo!()
}

pub unsafe fn preempt_disable() {
    todo!()
}

pub unsafe fn preempt_enable() -> bool {
    todo!()
}

pub unsafe fn switch(from: *const Task, to: *const Task) {
    todo!()
}

pub unsafe fn remote_reschedule(cpu: u32) {
    todo!()
}

pub fn init_task(
    task: &mut TaskContext,
    entry: extern "C" fn(usize, usize),
    arg1: usize,
    arg2: usize,
    stack_start: VirtAddr,
    is_user: bool,
) -> EResult<()> {
    todo!()
}

pub unsafe fn jump_to_user(ip: VirtAddr, sp: VirtAddr) {
    todo!()
}

pub unsafe fn jump_to_user_context(context: *mut Context) {
    todo!()
}
