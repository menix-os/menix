// Scheduling and context management for x86_64

use super::Context;
use crate::{pop_all_regs, swapgs_if_necessary};
use core::arch::asm;

/// Finalizes a context switch for an interrupt. This function may only be called by the scheduler.
#[naked]
unsafe extern "C" fn context_switch_finalize(context: *mut Context) {
    // ! Note:
    // The old context state gets saved by the interrupt stub. The only thing
    // that still needs to be done is to pop the new state back.
    // Usually, this would be done by the interrupt stub, but since we also pass
    // RDI, we return directly to avoid any accidental register clobbering.

    unsafe {
        asm!(
            "mov rsp, rdi",  // First argument is a reference to the thread's CpuRegisters field.
            pop_all_regs!(), // Pop all values stored in that struct into the actual registers.
            "add rsp, 0x18", // Skip .error, .isr and .core fields. (3 * 8 bytes) so we can check if we have to swapgs.
            swapgs_if_necessary!(), // Swap GSBASE to user mode.
            // Instead of returning via interrupt_internal, return directly so we always land in user mode.
            "iretq",
            options(noreturn)
        );
    }
}
