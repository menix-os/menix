#![allow(unused)]

use crate::arch::Cpu;

use super::idt::{IdtRegister, IDT_SIZE};
use super::VirtAddr;
use super::{gdt::GdtRegister, Context};
use core::arch::x86_64::__cpuid_count;
use core::arch::{asm, global_asm};

/// Wrapper for the `lgdt` instruction.
/// Only changing the GDT on its own is technically unsafe.
pub unsafe fn lgdt(gdt: &GdtRegister) {
    unsafe {
        asm!("lgdt [{0}]", in(reg) gdt);
    }
}

/// Wrapper for the `lidt` instruction.
/// Only changing the IDT on its own is technically unsafe.
pub unsafe fn lidt(idt: &IdtRegister) {
    unsafe {
        asm!("lidt [{0}]", in(reg) idt);
    }
}

/// Wrapper for the `cpuid` instruction.
pub fn cpuid(a: &mut usize, b: &mut usize, c: &mut usize, d: &mut usize) {
    unsafe {
        let result = __cpuid_count(*c as u32, *a as u32);
        *a = result.eax as usize;
        *b = result.ebx as usize;
        *c = result.ecx as usize;
        *d = result.edx as usize;
    }
}

/// Writes an unsigned 64-bit value to a model-specific register.
pub unsafe fn wrmsr(msr: u32, value: u64) {
    unsafe {
        let eax = value as u32;
        let edx = (value >> 32) as u32;
        asm!("wrmsr", in("eax") eax, in("edx") edx, in("ecx") msr);
    }
}

/// Writes an unsigned 64-bit value to the model-specific XCR register.
pub unsafe fn wrxcr(msr: u32, value: u64) {
    unsafe {
        let eax = value as u32;
        let edx = (value >> 32) as u32;
        asm!("xsetbv", in("eax") eax, in("edx") edx, in("ecx") msr);
    }
}

/// Reads an unsigned 64-bit value from a model-specific register.
pub unsafe fn rdmsr(msr: u32) -> u64 {
    unsafe {
        let eax: u32;
        let edx: u32;
        asm!("wrmsr", out("eax") eax, out("edx") edx, in("ecx") msr);
        return (eax as u64) | ((edx as u64) << 32);
    }
}

/// Saves the FPU state to a 512-byte region of memory using FXSAVE.
/// Pointer must be 16-byte aligned.
pub unsafe fn fxsave(memory: *mut u8) {
    unsafe {
        asm! ("fxsave {0}", in(reg) memory);
    }
}

/// Swaps GSBASE if CPL is 3.
#[macro_export]
macro_rules! swapgs_if_necessary {
    () => {
        concat!("cmp word ptr [rsp+0x8], 0x8;", "je 2f;", "swapgs;", "2:")
    };
}

/// Pushes all general purpose registers onto the stack.
#[macro_export]
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
#[macro_export]
macro_rules! pop_all_regs {
    () => {
        concat!(
            "pop rax;", "pop rbx;", "pop rcx;", "pop rdx;", "pop rbp;", "pop rdi;", "pop rsi;",
            "pop r8;", "pop r9;", "pop r10;", "pop r11;", "pop r12;", "pop r13;", "pop r14;",
            "pop r15;"
        )
    };
}

pub unsafe extern "C" fn interrupt_disable() {
    unsafe { asm!("cli") };
}

pub unsafe extern "C" fn interrupt_enable() {
    unsafe { asm!("sti") };
}
