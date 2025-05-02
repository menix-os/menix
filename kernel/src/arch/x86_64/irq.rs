use super::consts::CPL_USER;
use super::task::TaskFrame;
use crate::arch::x86_64::gdt::Gdt;
use crate::generic;
use crate::generic::exec::Frame;
use crate::generic::memory::page::{PageFaultCause, PageFaultInfo};
use crate::generic::{irq::IrqController, memory::VirtAddr, percpu::CpuData, syscall};
use core::arch::asm;
use core::fmt::Display;
use core::{arch::naked_asm, mem::offset_of};
use seq_macro::seq;

/// Registers which are saved and restored during a context switch or interrupt.
#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct TrapFrame {
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
static_assert!(size_of::<TrapFrame>() == 0xB0);

impl TrapFrame {
    pub const fn new() -> Self {
        Self {
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            r11: 0,
            r10: 0,
            r9: 0,
            r8: 0,
            rsi: 0,
            rdi: 0,
            rbp: 0,
            rdx: 0,
            rcx: 0,
            rbx: 0,
            rax: 0,
            isr: 0,
            error: 0,
            rip: 0,
            cs: 0,
            rflags: 0,
            rsp: 0,
            ss: 0,
        }
    }
}

impl Frame for TrapFrame {
    fn set_stack(&mut self, addr: usize) {
        self.rsp = addr as u64;
    }

    fn set_ip(&mut self, addr: usize) {
        self.rip = addr as u64;
    }

    fn get_stack(&self) -> usize {
        self.rsp as usize
    }

    fn get_ip(&self) -> usize {
        self.rip as usize
    }

    fn save(&self) -> TaskFrame {
        TaskFrame {
            r15: self.r15,
            r14: self.r14,
            r13: self.r13,
            r12: self.r12,
            r11: self.r11,
            r10: self.r10,
            r9: self.r9,
            r8: self.r8,
            rsi: self.rsi,
            rdi: self.rdi,
            rbp: self.rbp,
            rdx: self.rdx,
            rcx: self.rcx,
            rbx: self.rbx,
            rax: self.rax,
            rip: self.rip,
            rsp: self.rsp,
            rflags: self.rflags,
        }
    }

    fn restore(&mut self, saved: TaskFrame) {
        todo!()
    }
}

/// Invoked by an interrupt stub. Its only job is to call the platform independent syscall handler.
unsafe extern "C" fn interrupt_handler(isr: usize, context: *mut TrapFrame) -> *mut TrapFrame {
    let mut result = context;
    unsafe {
        match isr as u8 {
            // Exceptions.
            0x0E => page_fault_handler(context),
            // Unhandled exceptions.
            0x00..0x20 => {
                error!("Registers:\n{:?}", *context);
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
                let cpu = &super::CPU_DATA.get(CpuData::get());
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
unsafe extern "C" fn syscall_handler(context: *mut TrapFrame) {
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

pub unsafe fn page_fault_handler(context: *const TrapFrame) {
    let mut cr2 = 0usize;
    unsafe {
        asm!("mov {cr2}, cr2", cr2 = out(reg) cr2);

        let mut cause = PageFaultCause::empty();
        let err = (*context).error;
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
            caused_by_user: (*context).cs & super::consts::CPL_USER as u64
                == super::consts::CPL_USER as u64,
            ip: ((*context).rip as usize).into(),
            addr: cr2.into(),
            cause,
        };
        return generic::memory::page::page_fault_handler(context.as_ref().unwrap(), &info);
    }
}

unsafe extern "C" fn timer_handler(context: *mut TrapFrame) -> *mut TrapFrame {
    let lapic = unsafe { super::apic::LAPIC.get(CpuData::get()) };
    let result = generic::exec::sched::reschedule(&(unsafe { *context }).save());
    lapic.eoi();
    return context;
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
            "mov rdi, rsp",               // Put the trap frame struct as first argument.
            "call {syscall_handler}",     // Call syscall handler
            pop_all_regs!(),              // Pop stack values back to the general purpose registers.
            "add rsp, 0x10",              // Skip .error and .isr fields (2 * sizeof(u64))
            "mov rsp, gs:{user_stack}",   // Load user stack from `Cpu.user_stack`.
            "swapgs",                     // Change GS to user mode.
            "sti",                        // Resume interrupts.
            "sysretq",                    // Return to user mode.

            syscall_handler = sym syscall_handler,
            user_stack = const offset_of!(CpuData, user_stack),
            kernel_stack = const offset_of!(CpuData, kernel_stack),
            user_code64 = const offset_of!(Gdt, user_code64) | CPL_USER as usize,
            user_data = const offset_of!(Gdt, user_data) | CPL_USER as usize,
        );
    }
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
#[unsafe(naked)]
unsafe extern "C" fn interrupt_stub_internal() {
    unsafe {
        naked_asm!(
            swapgs_if_necessary!(),     // Load the kernel GS base.
            push_all_regs!(),           // Push all general purpose registers.
            "xor rbp, rbp",             // Zero out the base pointer since we can't trust it.
            "mov rdi, [rsp + 0x78]",    // Load the ISR value we pushed in the stub.
            "mov rsi, rsp",             // Load the frame as second argument.
            "call {interrupt_handler}", // Call interrupt handler.
            "mov rsp, rax",             // Restore the returned frame.
            pop_all_regs!(),            // Pop all general purpose registers.
            swapgs_if_necessary!(),     // Change GS back if we came from user mode.
            "add rsp, 0x10",            // Skip .error and .isr fields.
            "iretq",                    // Leave.
            interrupt_handler = sym interrupt_handler
        );
    }
}

pub unsafe fn set_irq_mask(mask: bool) -> bool {
    let old_mask = get_irq_state();
    unsafe {
        if mask {
            asm!("cli");
        } else {
            asm!("sti");
        }
    }
    return old_mask;
}

pub fn get_irq_state() -> bool {
    let mut flags: u64;
    unsafe {
        asm!("pushf; pop {0}", out(reg) flags);
    }
    return flags & (super::consts::RFLAGS_IF as u64) == 0;
}

pub fn wait_for_irq() {
    unsafe {
        asm!("hlt");
    }
}
