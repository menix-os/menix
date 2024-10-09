use super::{idt::IDT_SIZE, Context, VirtAddr};
use crate::{arch::Cpu, pop_all_regs, push_all_regs, swapgs_if_necessary, syscall};
use core::{
    arch::{asm, global_asm},
    mem::offset_of,
};

/// Invoked by an interrupt stub. Its only job is to call the platform independent syscall handler.
unsafe extern "C" fn interrupt_handler(context: *mut Context) {
    unsafe {
        match (*context).isr {
            // Legacy syscall invocation.
            0x80 => syscall_handler(context),
            _ => todo!(),
        }
    }
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
        asm!(
            "cli",            // Disable interrupts.
            "swapgs",         // Change GS to kernel mode.
            "mov gs:16, rsp", // Save user stack to `Cpu.user_stack`.
            "mov rsp, gs:8",  // Restore kernel stack from `Cpu.kernel_stack`.
            "cld",            // Clear direction bit from RFLAGS
            // We're pretending to be an interrupt, so fill the bottom fields of CpuRegisters.
            // For details see: https://www.felixcloutier.com/x86/syscall
            "push 0x23", // SS and CS are not changed during SYSCALL. Use `gdt_table.user_data | CPL_USER`.
            "push gs:16", // Get RSP from when we saved it
            "push r11",  // RFLAGS is moved into r11 by the CPU.
            "push 0x2b", // Same as SS. Use `gdt_table.user_code64 | CPL_USER`
            "push rcx",  // RIP is moved into rcx by the CPU.
            "push 0x00", // Context::error field
            "push 0x00", // Context::isr field
            "push 0x00", // Context::core field
            push_all_regs!(), // Push general purpose registers so they can be written to by syscalls.
            "mov rdi, rsp",   // Put `*mut Context` as first argument.
            "call {syscall_handler}", // Call syscall handler
            pop_all_regs!(),  // Pop stack values back to the general purpose registers.
            "add rsp, 0x18",  // Skip .error, .isr and .core fields
            "mov rsp, gs:16", // Load user stack from `Cpu.user_stack`.
            "swapgs",         // Change GS to user mode.
            "sti",            // Resume interrupts.
            "sysretq",        // Return to user mode.

            syscall_handler = sym syscall_handler,
            options(noreturn)
        );
    }
}

// Interrupt stub generation.
// There are some interrupts which generate an error code on the stack, while others do not.
// We streamline this difference by just pushing 0 for those that don't generate an error code.
// That means that we need to have two small stubs that slightly differ, and invoke a common handler.
global_asm!(
    // Macro to create interrupt stubs that push 0 as the error code.
    ".macro interrupt_stub num",
    ".align 0x10",
    "interrupt_\\num:",
        swapgs_if_necessary!(), // Change GS to kernel mode if we're coming from user mode.
        // The error code is
        "push 0",
        "push \\num",
        "jmp {interrupt_stub_internal}",
        ".endm",

    // Macro to create interrupt stubs with an actual error code.
    ".macro interrupt_stub_err num",
    ".align 0x10",
    "interrupt_\\num:",
        swapgs_if_necessary!(), // Change GS to kernel mode if we're coming from user mode.
        // The error code has already been pushed for us by the CPU, now just push the ISR number.
        "push \\num",
        "jmp {interrupt_stub_internal}",
        ".endm",

    // Define 256 interrupt stubs using the macros above.
    ".extern interrupt_handler",
    ".altmacro",
    ".set i, 0",
    ".rept 256",
        ".if (i == 8 || (i >= 10 && i <= 14) || i == 17 || i == 21 || i == 29 || i == 30)",
            "interrupt_stub_err %i",
        ".else",
            "interrupt_stub %i",
        ".endif",
        ".set i, i+1",
    ".endr",

    ".macro interrupt_num num",
        ".8byte interrupt_\\num",
    ".endm",

    // Put all handlers in one array so we can call it from a Rust function.
    ".global {interrupt_array}",
    "{interrupt_array}:",
    ".set i, 0",
    ".rept 256",
        "interrupt_num %i",
        ".set i, i+1",
    ".endr",

    interrupt_array = sym INTERRUPT_ARRAY,
    interrupt_stub_internal = sym interrupt_stub_internal
);

extern "C" {
    /// An array of interrupt stubs. They should not be called directly from Rust,
    /// hence the VirtAddr type instead of extern fn().
    pub static INTERRUPT_ARRAY: [VirtAddr; IDT_SIZE];
}

#[naked]
unsafe extern "C" fn interrupt_stub_internal() {
    unsafe {
        asm!(
            "push gs:{cpu_id}",         // Push CPU ID.
            push_all_regs!(),           // Push all general purpose registers.
            "mov rdi, rsp",             // Load the `*mut Context` as first argument.
            "xor rbp, rbp",             // Zero out the base pointer since we can't trust it.
            "call {interrupt_handler}", // Call interrupt handler.
            pop_all_regs!(),            // Pop all general purpose registers.
            "add rsp, 0x18",            // Skip .error, .isr, and .core fields.
            swapgs_if_necessary!(),     // Change GS back if we came from user mode.
            "iretq",                    // Leave.

            cpu_id = const offset_of!(Cpu, id),
            interrupt_handler = sym interrupt_handler,
            options(noreturn)
        );
    }
}
