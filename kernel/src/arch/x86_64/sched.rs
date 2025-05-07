use super::irq::TrapFrame;
use crate::generic::{
    memory::VirtAddr,
    percpu::CpuData,
    sched::task::{Frame, Task},
};
use core::{arch::asm, mem::offset_of};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Context {
    pub frame: TrapFrame,
    pub fsbase: u64,
    pub gsbase: u64,
    pub saved_fpu: VirtAddr,
}

impl Frame for Context {
    fn set_stack(&mut self, addr: usize) {
        self.frame.rsp = addr as u64;
    }

    fn get_stack(&self) -> usize {
        self.frame.rsp as usize
    }

    fn set_ip(&mut self, addr: usize) {
        self.frame.rip = addr as u64;
    }

    fn get_ip(&self) -> usize {
        self.frame.rip as usize
    }
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

pub fn reschedule_now() {
    todo!();
}

pub fn switch(from: *mut Task, to: *mut Task) {
    todo!()
}
