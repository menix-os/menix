use super::{consts, sched::Context};
use crate::arch::x86_64::consts::CPL_USER;
use crate::{
    arch::x86_64::system::gdt::Gdt,
    generic::{self, percpu::CpuData},
};
use core::{
    arch::{asm, naked_asm},
    mem::offset_of,
};

pub(in crate::arch) unsafe fn set_irq_state(value: bool) -> bool {
    let old_mask = get_irq_state();
    unsafe {
        if value {
            asm!("sti", options(nostack));
        } else {
            asm!("cli", options(nostack));
        }
    }
    return old_mask;
}

pub(in crate::arch) fn get_irq_state() -> bool {
    let mut flags: u64;
    unsafe {
        asm!("pushf; pop {0}", out(reg) flags);
    }
    return flags & (consts::RFLAGS_IF as u64) != 0;
}

pub(in crate::arch) fn wait_for_irq() {
    unsafe {
        asm!("hlt", options(nostack));
    }
}

/// Handles a syscall via AMD64 syscall/sysret instructions.
/// # Safety
/// Assumes that a valid stack is ready in the PerCpu block at this point.
#[unsafe(naked)]
pub unsafe extern "C" fn amd64_syscall_stub() {
    naked_asm!(
        "swapgs",
        "mov gs:{user_stack}, rsp",
        "mov rsp, gs:{kernel_stack}",
        "cld",
        // We're pretending to be an interrupt, so fill the bottom fields of `Context`.
        "push {user_data}",           // SS and CS are not changed during SYSCALL. Use `Gdt::user_data | CPL_USER`.
        "push gs:{user_stack}",
        "push r11",                   // RFLAGS is moved into r11 by the CPU.
        "push {user_code64}",         // Same as SS. Use `Gdt::user_code64 | CPL_USER`
        "push rcx",                   // RIP is moved into rcx by the CPU.
        "push 0x00",                  // Context::error field
        "push 0x00",                  // Context::isr field
        "push rax",
        "push rbx",
        "push rcx",
        "push rdx",
        "push rbp",
        "push rdi",
        "push rsi",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "xor rbp, rbp",
        "mov rdi, rsp",               // Put the trap frame struct as first argument.
        "call {syscall_handler}",     // Call syscall handler
        "cli",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rsi",
        "pop rdi",
        "pop rbp",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",
        "add rsp, 0x10",              // Skip .error and .isr fields (2 * sizeof(u64))
        "mov rsp, gs:{user_stack}",   // Load user stack from `Cpu.user_stack`.
        "swapgs",
        "sysretq",                    // Return to user mode.

        syscall_handler = sym syscall_handler,
        user_stack = const offset_of!(CpuData, user_stack),
        kernel_stack = const offset_of!(CpuData, kernel_stack),
        user_code64 = const offset_of!(Gdt, user_code64) | CPL_USER as usize,
        user_data = const offset_of!(Gdt, user_data) | CPL_USER as usize,
    );
}

/// Invoked by either the interrupt or syscall stub.
extern "C" fn syscall_handler(frame: *mut Context) {
    unsafe {
        // Arguments use the SYSV C ABI.
        // Except for a3, since RCX is needed for sysret, we need a different register.
        let result = generic::syscall::dispatch(
            (*frame).rax as usize,
            (*frame).rdi as usize,
            (*frame).rsi as usize,
            (*frame).rdx as usize,
            (*frame).r10 as usize,
            (*frame).r8 as usize,
            (*frame).r9 as usize,
        );
        (*frame).rax = result.0 as u64;
        (*frame).rdx = result.1 as u64;
    }
}
