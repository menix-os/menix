use seq_macro::seq;

use super::{consts::CPL_USER, gdt::Gdt, idt::IDT_SIZE, sched::Context, VirtAddr};
use crate::{arch::Cpu, pop_all_regs, push_all_regs, swapgs_if_necessary, syscall};
use core::{
    arch::{asm, global_asm, naked_asm},
    mem::offset_of,
};

/// Invoked by an interrupt stub. Its only job is to call the platform independent syscall handler.
unsafe extern "C" fn interrupt_handler(context: *mut Context) -> *mut Context {
    unsafe {
        match (*context).isr {
            // Legacy syscall invocation.
            0x80 => syscall_handler(context),
            _ => (),
        };
    }
    return context;
}

/// Invoked by either the interrupt or syscall stub.
unsafe extern "C" fn syscall_handler(context: *mut Context) {
    unsafe {
        // Arguments use the SYSV C ABI.
        (*context).rax = syscall::invoke(
            (*context).rax as usize,
            (*context).rdi as usize,
            (*context).rsi as usize,
            (*context).rdx as usize,
            (*context).r10 as usize,
            (*context).r8 as usize,
            (*context).r9 as usize,
        ) as u64
    }
}

/// Handles a syscall via AMD64 syscall/sysret instructions.
#[naked]
unsafe extern "C" fn amd64_syscall_stub() {
    unsafe {
        naked_asm!(
            "cli",                        // Disable interrupts.
            "swapgs",                     // Change GS to kernel mode.
            "mov gs:{user_stack}, rsp",   // Save user stack to `Cpu.user_stack`.
            "mov rsp, gs:{kernel_stack}", // Restore kernel stack from `Cpu.kernel_stack`.
            "cld",                        // Clear direction bit from RFLAGS
            // We're pretending to be an interrupt, so fill the bottom fields of `Context`.
            // For details see: https://www.felixcloutier.com/x86/syscall
            "push {user_data}",           // SS and CS are not changed during SYSCALL. Use `Gdt::user_data | CPL_USER`.
            "push gs:{user_stack}",       // Save the user stack pointer.
            "push r11",                   // RFLAGS is moved into r11 by the CPU.
            "push {user_code64}",         // Same as SS. Use `Gdt::user_code64 | CPL_USER`
            "push rcx",                   // RIP is moved into rcx by the CPU.
            "push 0x00",                  // Context::error field
            "push 0x00",                  // Context::isr field
            "push 0x00",                  // Context::core field
            push_all_regs!(),             // Push general purpose registers so they can be written to by syscalls.
            "mov rdi, rsp",               // Put `*mut Context` as first argument.
            "call {syscall_handler}",     // Call syscall handler
            pop_all_regs!(),              // Pop stack values back to the general purpose registers.
            "add rsp, 0x18",              // Skip .error, .isr and .core fields (3 * sizeof(u64))
            "mov rsp, gs:{user_stack}",   // Load user stack from `Cpu.user_stack`.
            "swapgs",                     // Change GS to user mode.
            "sti",                        // Resume interrupts.
            "sysretq",                    // Return to user mode.

            syscall_handler = sym syscall_handler,
            user_stack = const offset_of!(Cpu, user_stack),
            kernel_stack = const offset_of!(Cpu, kernel_stack),
            user_code64 = const offset_of!(Gdt, user_code64) | CPL_USER as usize,
            user_data = const offset_of!(Gdt, user_data) | CPL_USER as usize,
        );
    }
}

// Interrupt stub generation.
// There are some interrupts which generate an error code on the stack, while others do not.
// We streamline this difference by just pushing 0 for those that don't generate an error code.
// That means that we need to have two small stubs that slightly differ, and invoke a common handler.
seq! { N in 0..256 {
    #[naked]
    pub(crate) unsafe extern "C" fn interrupt_stub~N() {
        unsafe {
            naked_asm!(
                // These codes push an error on the stack.
                ".if ({i} == 8 || ({i} >= 10 && {i} <= 14) || {i} == 17 || {i} == 21 || {i} == 29 || {i} == 30)",
                // All other ones don't, so we need to push something ourselves.
                ".else",
                "push 0",
                ".endif",

                "push {i}",
                "jmp {interrupt_stub_internal}",

                i = const N,
                interrupt_stub_internal = sym interrupt_stub_internal
            );
        }
    }
}}

#[naked]
unsafe extern "C" fn interrupt_stub_internal() {
    unsafe {
        naked_asm!(
            "push gs:{cpu_id}",         // Push CPU ID.
            push_all_regs!(),           // Push all general purpose registers.
            "xor rbp, rbp",             // Zero out the base pointer since we can't trust it.
            "mov rdi, rsp",             // Load the `*mut Context` as first argument.
            "call {interrupt_handler}", // Call interrupt handler.
            "mov rsp, rax",             // Restore the returned `*mut Context`.
            pop_all_regs!(),            // Pop all general purpose registers.
            "add rsp, 0x18",            // Skip .error, .isr, and .core fields.
            swapgs_if_necessary!(),     // Change GS back if we came from user mode.
            "iretq",                    // Leave.

            cpu_id = const offset_of!(Cpu, id),
            interrupt_handler = sym interrupt_handler
        );
    }
}