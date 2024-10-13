// Scheduling and context management for x86_64

use crate::{arch::CommonContext, pop_all_regs, swapgs_if_necessary};
use core::arch::naked_asm;

/// Registers which are saved and restored during a context switch or interrupt.
#[repr(C, packed)]
#[derive(Clone, Debug, Default)]
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
    pub core: u64,
    pub isr: u64,
    // Pushed onto the stack by the CPU if the interrupt has an error code.
    pub error: u64,
    // Pushed onto the stack by the CPU during an interrupt.
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl CommonContext for Context {}

/// Finalizes a context switch for an interrupt. This function may only be called by the scheduler.
#[naked]
unsafe extern "C" fn context_switch_finalize(context: *mut Context) {
    // ! Note:
    // The old context state gets saved by the interrupt stub. The only thing
    // that still needs to be done is to pop the new state back.
    // Usually, this would be done by the interrupt stub, but since we also pass
    // RDI, we return directly to avoid any accidental register clobbering.

    unsafe {
        naked_asm!(
            "mov rsp, rdi",  // First argument is a reference to the thread's CpuRegisters field.
            pop_all_regs!(), // Pop all values stored in that struct into the actual registers.
            "add rsp, 0x18", // Skip .error, .isr and .core fields. (3 * 8 bytes) so we can check if we have to swapgs.
            swapgs_if_necessary!(), // Swap GSBASE to user mode.
            // Instead of returning via interrupt_internal, return directly so we always land in user mode.
            "iretq"
        );
    }
}
