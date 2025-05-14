use super::{consts::CPL_USER, core::get_per_cpu, platform::gdt::Gdt};
use crate::{
    arch::{
        self,
        x86_64::{asm::wrmsr, consts},
    },
    generic::{percpu::CpuData, sched::task::Task},
};
use core::{arch::asm, mem::offset_of};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct TaskContext {
    pub rbx: u64,
    pub rbp: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

#[repr(C)]
#[derive(Default, Clone, Debug, Copy)]
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
    // Pushed onto the stack by the interrupt handler stubs.
    pub isr: u64,
    // Pushed onto the stack by the CPU if the interrupt has an error code.
    pub error: u64,
    // The rest is pushed onto the stack by the CPU during an interrupt.
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}
static_assert!(size_of::<Context>() == 22 * size_of::<u64>());

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

pub fn switch(from: *mut Task, to: *mut Task) {}

pub unsafe fn jump_to_user(ip: usize, sp: usize) -> ! {
    unsafe {
        assert!(
            (*get_task()).is_user(),
            "Attempted to perform a user jump on a kernel task!"
        );
    }

    // Create a new context for the user jump.
    let mut context = Context::default();
    context.rip = ip as u64;
    context.rsp = sp as u64;
    context.rflags = 0x200;
    //context.cs = offset_of!(Gdt, user_code64) as u64 | CPL_USER as u64;
    //context.ss = offset_of!(Gdt, user_data) as u64 | CPL_USER as u64;
    context.cs = offset_of!(Gdt, kernel_code) as u64;
    context.ss = offset_of!(Gdt, kernel_data) as u64;

    // Clear segment registers. Because this also clears GSBASE, we have to restore it immediately.
    unsafe {
        let old_irq_state = arch::irq::set_irq_state(false);
        let percpu = get_per_cpu();

        let zero = 0u16;
        asm!("mov ds, {zero:x}", "mov es, {zero:x}", "mov fs, {zero:x}", "mov gs, {zero:x}", zero = in(reg) zero);

        wrmsr(consts::MSR_FS_BASE, 0);
        wrmsr(consts::MSR_GS_BASE, percpu as u64);
        wrmsr(consts::MSR_KERNEL_GS_BASE, 0);

        arch::irq::set_irq_state(old_irq_state);
        jump_to_user_context(&raw mut context);
    }
}

pub unsafe fn jump_to_user_context(context: *mut Context) -> ! {
    unsafe {
        assert!(
            (*get_task()).is_user(),
            "Attempted to perform a user jump on a kernel task!"
        );

        asm!(
            "mov rsp, {context}",
            "jmp {interrupt_return}",
            context = in(reg) context,
            interrupt_return = sym super::irq::interrupt_return
        );

        unreachable!();
    }
}
