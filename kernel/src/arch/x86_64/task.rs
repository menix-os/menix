use crate::generic::{exec::Frame, memory::VirtAddr};
use core::fmt::Display;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TaskFrame {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
}

impl TaskFrame {
    pub fn new() -> Self {
        Self {
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            r11: 0,
            r10: 0,
            r9: 0,
            r8: 0,
            rsi: 0,
            rdi: 0,
            rbp: 0,
            rdx: 0,
            rcx: 0,
            rbx: 0,
            rax: 0,
            rip: 0,
            rsp: 0,
            rflags: 0,
        }
    }
}

impl Frame for TaskFrame {
    fn set_stack(&mut self, addr: usize) {
        self.rsp = addr as u64;
    }

    fn get_stack(&self) -> usize {
        self.rsp as usize
    }

    fn set_ip(&mut self, addr: usize) {
        self.rip = addr as u64;
    }

    fn get_ip(&self) -> usize {
        self.rip as usize
    }

    fn save(&self) -> TaskFrame {
        self.clone()
    }

    fn restore(&mut self, saved: TaskFrame) {
        *self = saved;
    }
}
