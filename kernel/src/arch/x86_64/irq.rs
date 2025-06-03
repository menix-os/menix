use super::{consts, sched::Context};
use crate::{
    arch::x86_64::platform::gdt::Gdt,
    generic::{self, percpu::CpuData},
};
use crate::{
    arch::x86_64::{ARCH_DATA, consts::CPL_USER},
    generic::{
        irq::IrqError,
        memory::virt::{PageFaultCause, PageFaultInfo},
    },
};
use core::{
    arch::{asm, naked_asm},
    mem::offset_of,
};
use seq_macro::seq;

/// Invoked by an interrupt stub. Its only job is to call the platform independent syscall handler.
unsafe extern "C" fn interrupt_handler(context: *mut Context) {
    let isr = (unsafe { *context }).isr as usize;

    match isr as u8 {
        // Exceptions.
        0x0E => page_fault_handler(context),
        // Unhandled exceptions.
        0x00..0x20 => {
            error!(
                "Got an unhandled CPU exception: {} (ISR {})!",
                match isr {
                    0x00 => "Division Error",
                    0x01 => "Debug",
                    0x02 => "NMI",
                    0x03 => "Breakpoint",
                    0x04 => "Overflow",
                    0x05 => "Bound Range Exceeded",
                    0x06 => "Invalid Opcode",
                    0x07 => "Device Not Available",
                    0x08 => "Double Fault",
                    0x0A => "Invalid TSS",
                    0x0B => "Segment Not Present",
                    0x0C => "Stack-Segment Fault",
                    0x0D => "General Protection Fault",
                    0x0E => "Page Fault",
                    0x10 => "x87 Floating Point Exception",
                    0x11 => "Alignment Check",
                    0x12 => "Machine Check",
                    0x13 => "SIMD Floating Point Exception",
                    0x14 => "Virtualization Exception",
                    0x15 => "Control Protection Exception",
                    0x1C => "Hypervisor Injection Exception",
                    0x1D => "VMM Communication Exception",
                    0x1E => "Security Exception",
                    _ => "Reserved",
                },
                isr
            );
            unsafe { print_context(context) };
            panic!();
        }
        // Any other ISR is an IRQ with a dynamic handler.
        _ => {
            match super::ARCH_DATA.get().irq_handlers[isr] {
                Some(x) => generic::irq::dispatch(x),
                None => {
                    panic!("Got an unhandled interrupt {}!", isr);
                }
            };
        }
    };
}

unsafe fn print_context(context: *const Context) {
    unsafe {
        error!("{:?}", *context);
    }
}

/// Invoked by either the interrupt or syscall stub.
extern "C" fn syscall_handler(frame: *mut Context) {
    unsafe {
        // Arguments use the SYSV C ABI.
        // Except for a3, since RCX is needed for sysret, we need a different register.
        let result = generic::sys::dispatch(
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

fn page_fault_handler(frame: *mut Context) {
    unsafe {
        let mut cr2: usize;
        asm!("mov {cr2}, cr2", cr2 = out(reg) cr2);

        let mut cause = PageFaultCause::empty();
        let err = (*frame).error;
        if err & (1 << 0) != 0 {
            cause |= PageFaultCause::Present;
        }
        if err & (1 << 1) != 0 {
            cause |= PageFaultCause::Write;
        }
        if err & (1 << 2) != 0 {
            cause |= PageFaultCause::User;
        }
        if err & (1 << 4) != 0 {
            cause |= PageFaultCause::Fetch;
        }

        let info = PageFaultInfo {
            caused_by_user: (*frame).cs & consts::CPL_USER as u64 == consts::CPL_USER as u64,
            ip: ((*frame).rip as usize).into(),
            addr: cr2.into(),
            cause,
        };

        generic::memory::virt::page_fault_handler(&info);
    }
}

pub(in crate::arch) unsafe fn set_irq_state(value: bool) -> bool {
    let old_mask = get_irq_state();
    unsafe {
        if value {
            asm!("sti");
        } else {
            asm!("cli");
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
        asm!("hlt");
    }
}

pub(in crate::arch) fn register_irq(irq: usize) -> Result<(), IrqError> {
    let cpu = ARCH_DATA.get();
    for (idx, isr) in cpu.irq_handlers.iter_mut().enumerate() {
        // ISR0-31 are reserved for exceptions.
        if isr.is_some() || idx < 0x20 {
            continue;
        }

        *isr = Some(irq);
        return Ok(());
    }

    return Err(IrqError::NoIrqsLeft);
}

/// Pushes all general purpose registers onto the stack.
macro_rules! push_all_regs {
    () => {
        concat!(
            "push rax;",
            "push rbx;",
            "push rcx;",
            "push rdx;",
            "push rbp;",
            "push rdi;",
            "push rsi;",
            "push r8;",
            "push r9;",
            "push r10;",
            "push r11;",
            "push r12;",
            "push r13;",
            "push r14;",
            "push r15;",
        )
    };
}

/// Pops all general purpose registers from the stack.
macro_rules! pop_all_regs {
    () => {
        concat!(
            "pop r15;", "pop r14;", "pop r13;", "pop r12;", "pop r11;", "pop r10;", "pop r9;",
            "pop r8;", "pop rsi;", "pop rdi;", "pop rbp;", "pop rdx;", "pop rcx;", "pop rbx;",
            "pop rax;",
        )
    };
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
        push_all_regs!(),
        "mov rdi, rsp",               // Put the trap frame struct as first argument.
        "call {syscall_handler}",     // Call syscall handler
        "cli",
        pop_all_regs!(),
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

/// Swaps GSBASE if we're coming from user space.
macro_rules! swapgs_if_necessary {
    () => {
        concat!("cmp word ptr [rsp+24], 0x8;", "je 2f;", "swapgs;", "2:")
    };
}

// There are some interrupts which generate an error code on the stack, while others do not.
// We normalize this by just pushing 0 for those that don't generate an error code.
seq! { N in 0..256 {
    #[unsafe(naked)]
    pub(crate) unsafe extern "C" fn interrupt_stub~N() {
        naked_asm!(
            // These codes push an error on the stack. Do nothing.
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
}}

/// To avoid having 256 big functions with essentially the same logic,
/// this function is meant to do the actual heavy lifting.
#[unsafe(naked)]
unsafe extern "C" fn interrupt_stub_internal() {
    naked_asm!(
        swapgs_if_necessary!(),     // Load the kernel GS base.
        push_all_regs!(),           // Push all general purpose registers.
        "cld",                      // Clear direction flag.
        "xor rbp, rbp",             // Zero out the base pointer since we can't trust it.
        "mov rdi, rsp",             // Load the frame as second argument.
        "call {interrupt_handler}", // Call interrupt handler.
        "jmp {interrupt_return}",   // Leave.
        interrupt_handler = sym interrupt_handler,
        interrupt_return = sym interrupt_return
    );
}

/// Returns from an interrupt frame.
#[unsafe(naked)]
pub unsafe extern "C" fn interrupt_return() {
    naked_asm!(
        pop_all_regs!(),        // Pop all general purpose registers.
        swapgs_if_necessary!(), // Change GS back if we came from user mode.
        "add rsp, 0x10",        // Skip .error and .isr fields.
        "iretq",
    );
}
