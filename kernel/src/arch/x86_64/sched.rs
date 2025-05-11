use crate::generic::{memory::VirtAddr, percpu::CpuData, sched::task::Task};
use core::{arch::asm, mem::offset_of};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Context {
    pub rbx: u64,
    pub rbp: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub fsbase: u64,
    pub gsbase: u64,
    pub saved_fpu: VirtAddr,
}

pub fn get_task() -> *mut Task {
    unsafe {
        let task: *mut Task;
        asm!(
            "mov {cpu}, gs:[{this}]",
            cpu = out(reg) task,
            this = const offset_of!(CpuData, scheduler.current),
            options(nostack, preserves_flags),
        );
        return task;
    }
}

pub fn switch(from: *mut Task, to: *mut Task) {
    todo!()
}
