use super::{ARCH_DATA, core::get_per_cpu, platform::gdt::Gdt};
use crate::{
    arch::{
        self,
        x86_64::{asm::wrmsr, consts},
    },
    generic::{
        errno::Errno,
        memory::{
            pmm::{AllocFlags, FreeList, PageAllocator},
            virt::KERNEL_STACK_SIZE,
        },
        percpu::CpuData,
        sched::task::Task,
    },
};
use core::{
    arch::{asm, naked_asm},
    mem::offset_of,
};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct TaskContext {
    pub rsp: u64,
    pub fpu_region: *mut u8,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub fsbase: u64,
    pub gsbase: u64,
    pub restarted: bool,
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

/// The task frame consists of registers that the C ABI marks as callee-saved.
/// If we don't save them, these registers are lost during a context switch.
/// The order of these fields is important.
#[repr(C)]
struct TaskFrame {
    rbx: u64,
    rbp: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
}

pub unsafe fn switch(from: *const Task, to: *const Task) {
    unsafe {
        let cpu = ARCH_DATA.get(CpuData::get());
        cpu.tss.rsp0 = (*to).stack as u64 + KERNEL_STACK_SIZE as u64;

        if (*from).is_user() {
            let mut from_context = (*from).task_context.lock();
            (cpu.fpu_save)(from_context.fpu_region);
            from_context.ds = super::asm::read_ds();
            from_context.es = super::asm::read_es();
            from_context.fs = super::asm::read_fs();
            from_context.gs = super::asm::read_gs();
        }

        if (*to).is_user() {
            let mut to_context = (*to).task_context.lock();
            (cpu.fpu_restore)(to_context.fpu_region);
            to_context.ds = super::asm::read_ds();
            to_context.es = super::asm::read_es();
            to_context.fs = super::asm::read_fs();

            // If we have to change the GS segment we need to reload the MSR, otherwise we lose its value.
            if to_context.gs != super::asm::read_gs() {
                let percpu = get_per_cpu();
                super::asm::write_gs(to_context.gs);
                wrmsr(consts::MSR_GS_BASE, percpu as u64);
            }
            wrmsr(consts::MSR_FS_BASE, to_context.fsbase);
            // KERNEL_GS_BASE is the inactive base (swapped to during iretq/sysretq).
            wrmsr(consts::MSR_KERNEL_GS_BASE, to_context.gsbase);
        }

        // Because we will otherwise never be able to break out, forcefully unlock this mutex.
        let old_rsp = &raw mut ((*from).task_context.lock()).rsp;
        (*from).task_context.force_unlock(false);

        let new_rsp = (*to).task_context.lock().rsp;

        // TODO: This is probably still broken.
        asm!(
            "sub rsp, 0x30", // Make room for all regs (except RIP).
            "mov [rsp + {rbx}], rbx",
            "mov [rsp + {rbp}], rbp",
            "mov [rsp + {r12}], r12",
            "mov [rsp + {r13}], r13",
            "mov [rsp + {r14}], r14",
            "mov [rsp + {r15}], r15",
            "mov [{old_rsp}], rsp", // Save the old pointer.
            "mov rsp, {new_rsp}",   // Set the new pointer.
            "mov rbx, [rsp + {rbx}]",
            "mov rbp, [rsp + {rbp}]",
            "mov r12, [rsp + {r12}]",
            "mov r13, [rsp + {r13}]",
            "mov r14, [rsp + {r14}]",
            "mov r15, [rsp + {r15}]",
            "add rsp, 0x30",
            "ret", // This will conveniently move us to the RIP we put at this stack entry.
            old_rsp = in(reg) old_rsp,
            new_rsp = in(reg) new_rsp,
            rbx = const offset_of!(TaskFrame, rbx),
            rbp = const offset_of!(TaskFrame, rbp),
            r12 = const offset_of!(TaskFrame, r12),
            r13 = const offset_of!(TaskFrame, r13),
            r14 = const offset_of!(TaskFrame, r14),
            r15 = const offset_of!(TaskFrame, r15),
        );
        unreachable!();
    }
}

pub fn init_task(
    context: &mut TaskContext,
    entry: extern "C" fn(usize) -> !,
    arg: usize,
    stack_start: usize,
    is_user: bool,
) -> Result<(), Errno> {
    let cpu = ARCH_DATA.get(CpuData::get());
    // Prepare a dummy stack with an entry point function to return to.
    unsafe {
        let frame = ((stack_start + KERNEL_STACK_SIZE) as *mut TaskFrame).sub(1);
        (*frame).rbx = entry as u64;
        (*frame).r12 = arg as u64;
        (*frame).rip = task_entry_thunk as u64;
        context.rsp = frame as u64;

        if is_user {
            context.fpu_region = FreeList::alloc_bytes(cpu.fpu_size, AllocFlags::Zeroed)
                .map_err(|_| Errno::ENOMEM)?
                .as_hhdm();
            context.ds = super::asm::read_ds();
            context.es = super::asm::read_es();
            context.fs = super::asm::read_fs();
            context.gs = super::asm::read_gs();
            context.fsbase = super::asm::rdmsr(consts::MSR_FS_BASE);
            context.gsbase = super::asm::rdmsr(consts::MSR_KERNEL_GS_BASE);
        }
    }

    Ok(())
}

/// This function only calls [`task_entry`] by moving values from callee saved regs to use the C ABI.
#[unsafe(naked)]
unsafe extern "C" fn task_entry_thunk() -> ! {
    naked_asm!(
        "mov rdi, rbx",
        "mov rsi, r12",
        "jmp {task_thunk}",
        task_thunk = sym task_entry,
    );
}

/// Sets up some task fields and calls the entry point.
unsafe extern "C" fn task_entry(entry: extern "C" fn(usize) -> !, arg: usize) -> ! {
    (entry)(arg);
}

pub unsafe fn force_reschedule() {
    unsafe { asm!("int 0x20") }; // TODO: Don't hard code this.
}

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
    context.cs = offset_of!(Gdt, user_code64) as u64 | consts::CPL_USER as u64;
    context.ss = offset_of!(Gdt, user_data) as u64 | consts::CPL_USER as u64;

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
