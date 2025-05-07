use crate::generic::sched::task::Frame;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Context {
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

impl Frame for Context {
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

    fn save(&self) -> Context {
        *self
    }

    fn restore(&mut self, saved: Context) {
        *self = saved;
    }
}
