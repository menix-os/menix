use super::consts::CPL_USER;
use super::cpu;
use crate::arch::{self, x86_64::gdt::Gdt};
use crate::generic::irq::IrqController;
use crate::generic::memory::VirtAddr;
use crate::generic::{cpu::PerCpu, syscall};
use core::fmt::{Debug, Display};
use core::{arch::naked_asm, mem::offset_of};
use seq_macro::seq;

/// Registers which are saved and restored during a context switch or interrupt.
#[repr(C)]
#[derive(Clone, Debug, Copy, Default)]
pub struct InterruptFrame {
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
    // Pushed onto the stack by the CPU during an interrupt.
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}
assert_size!(InterruptFrame, 0xB0);

impl InterruptFrame {
    pub fn set_stack(&mut self, addr: VirtAddr) {
        self.rsp = addr.0 as u64;
    }

    pub fn set_ip(&mut self, addr: VirtAddr) {
        self.rip = addr.0 as u64;
    }
}

impl Display for InterruptFrame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "rax  {:016x} rbx  {:016x} rcx  {:016x} rdx  {:016x}\n\
             rbp  {:016x} rdi  {:016x} rsi  {:016x} r8   {:016x}\n\
             r9   {:016x} r10  {:016x} r11  {:016x} r12  {:016x}\n\
             r13  {:016x} r14  {:016x} r15  {:016x} rip  {:016x}\n\
             cs   {:016x} ss   {:016x} rflg {:016x} rsp  {:016x}",
            &self.rax,
            &self.rbx,
            &self.rcx,
            &self.rdx,
            &self.rbp,
            &self.rdi,
            &self.rsi,
            &self.r8,
            &self.r9,
            &self.r10,
            &self.r11,
            &self.r12,
            &self.r13,
            &self.r14,
            &self.r15,
            &self.rip,
            &self.cs,
            &self.ss,
            &self.rflags,
            &self.rsp,
        ))
    }
}

/// Invoked by an interrupt stub. Its only job is to call the platform independent syscall handler.
unsafe extern "C" fn interrupt_handler(
    isr: usize,
    context: *mut InterruptFrame,
) -> *mut InterruptFrame {
    let mut result = context;
    unsafe {
        match isr as u8 {
            // Exceptions.
            0x0E => _ = arch::page::page_fault_handler(context),
            // Unhandled exceptions.
            0..0x20 => {
                error!("Registers:\n{}\n", *context);
                panic!(
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
            }
            // Timer.
            0x20 => result = timer_handler(context),
            // Legacy Syscall.
            0x80 => syscall_handler(context),
            //
            _ => {
                let cpu = &PerCpu::get_per_cpu().arch;
                match cpu.irq_handlers[isr as usize] {
                    Some(x) => x(cpu.irq_map[isr as usize], cpu.irq_ctx[isr as usize]),
                    None => panic!("Got an unhandled interrupt {}!", isr),
                };
            }
        };
    }
    return result;
}

/// Invoked by either the interrupt or syscall stub.
unsafe extern "C" fn syscall_handler(context: *mut InterruptFrame) {
    unsafe {
        // Arguments use the SYSV C ABI.
        // Except for a3, since RCX is needed for sysret, we need a different register.
        let result = syscall::invoke(
            (*context).rax as usize,
            (*context).rdi as usize,
            (*context).rsi as usize,
            (*context).rdx as usize,
            (*context).r10 as usize,
            (*context).r8 as usize,
            (*context).r9 as usize,
        );
        (*context).rax = result.0 as u64;
        (*context).rdx = result.1 as u64;
    }
}

unsafe extern "C" fn timer_handler(context: *mut InterruptFrame) -> *mut InterruptFrame {
    let percpu = unsafe { PerCpu::get_per_cpu() };
    if let Some(lapic) = &percpu.arch.lapic {
        lapic.eoi();
    }
    return context;
}

/// Swaps GSBASE if CPL is 3.
macro_rules! swapgs_if_necessary {
    () => {
        concat!("cmp word ptr [rsp+0x8], 0x8;", "je 2f;", "swapgs;", "2:")
    };
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
            "push r15;"
        )
    };
}

/// Pops all general purpose registers from the stack.
macro_rules! pop_all_regs {
    () => {
        concat!(
            "pop rax;", "pop rbx;", "pop rcx;", "pop rdx;", "pop rbp;", "pop rdi;", "pop rsi;",
            "pop r8;", "pop r9;", "pop r10;", "pop r11;", "pop r12;", "pop r13;", "pop r14;",
            "pop r15;"
        )
    };
}

/// Handles a syscall via AMD64 syscall/sysret instructions.
/// # Safety
/// Assumes that a valid stack is ready in the PerCpu block at this point.
#[naked]
pub unsafe extern "C" fn amd64_syscall_stub() {
    unsafe {
        naked_asm!(
            "cli",                        // Disable interrupts.
            "swapgs",                     // Change GS to kernel mode.
            "mov gs:{user_stack}, rsp",   // Save user stack to `Cpu.user_stack`.
            "mov rsp, gs:{kernel_stack}", // Restore kernel stack from `Cpu.kernel_stack`.
            "cld",                        // Clear direction bit from RFLAGS
            // We're pretending to be an interrupt, so fill the bottom fields of `Context`.
            "push {user_data}",           // SS and CS are not changed during SYSCALL. Use `Gdt::user_data | CPL_USER`.
            "push gs:{user_stack}",       // Save the user stack pointer.
            "push r11",                   // RFLAGS is moved into r11 by the CPU.
            "push {user_code64}",         // Same as SS. Use `Gdt::user_code64 | CPL_USER`
            "push rcx",                   // RIP is moved into rcx by the CPU.
            "push 0x00",                  // Context::error field
            "push 0x00",                  // Context::isr field
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
            user_stack = const offset_of!(PerCpu, user_stack),
            kernel_stack = const offset_of!(PerCpu, kernel_stack),
            user_code64 = const offset_of!(Gdt, user_code64) | CPL_USER as usize,
            user_data = const offset_of!(Gdt, user_data) | CPL_USER as usize,
        );
    }
}

// Interrupt stub generation.

// There are some interrupts which generate an error code on the stack, while others do not.
// We normalize this by just pushing 0 for those that don't generate an error code.
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

// To avoid having 256 big functions with essentially the same logic,
// this function is meant to do the actual heavy lifting.
#[naked]
unsafe extern "C" fn interrupt_stub_internal() {
    unsafe {
        naked_asm!(
            push_all_regs!(),           // Push all general purpose registers.
            "xor rbp, rbp",             // Zero out the base pointer since we can't trust it.
            "mov rdi, [rsp + 0x78]",    // Load the ISR value we pushed in the stub.
            "mov rsi, rsp",             // Load the `*mut Context` as second argument.
            "call {interrupt_handler}", // Call interrupt handler.
            "mov rsp, rax",             // Restore the returned `*mut Context`.
            pop_all_regs!(),            // Pop all general purpose registers.
            "add rsp, 0x10",            // Skip .error and .isr fields.
            swapgs_if_necessary!(),     // Change GS back if we came from user mode.
            "iretq",                    // Leave.
            interrupt_handler = sym arch::irq::interrupt_handler
        );
    }
}
