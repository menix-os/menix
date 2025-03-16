#![allow(unused)]

use super::VirtAddr;
use super::gdt::GdtRegister;
use super::idt::{IDT_SIZE, IdtRegister};
use core::arch::x86_64::__cpuid_count;
use core::arch::{asm, global_asm};

pub struct CpuIdResult {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

/// Wrapper for the `cpuid` instruction.
#[inline]
pub fn cpuid(leaf: u32, sub_leaf: u32) -> CpuIdResult {
    unsafe {
        let result = __cpuid_count(leaf, sub_leaf);
        return CpuIdResult {
            eax: result.eax,
            ebx: result.ebx,
            ecx: result.ecx,
            edx: result.edx,
        };
    }
}

/// Writes an unsigned 64-bit value to a model-specific register.
#[inline]
pub fn wrmsr(msr: u32, value: u64) {
    unsafe {
        let eax = value as u32;
        let edx = (value >> 32) as u32;
        asm!("wrmsr", in("eax") eax, in("edx") edx, in("ecx") msr);
    }
}

/// Writes an unsigned 64-bit value to the model-specific XCR register.
#[inline]
pub fn wrxcr(msr: u32, value: u64) {
    unsafe {
        let eax = value as u32;
        let edx = (value >> 32) as u32;
        asm!("xsetbv", in("eax") eax, in("edx") edx, in("ecx") msr);
    }
}

/// Reads an unsigned 64-bit value from a model-specific register.
#[inline]
pub fn rdmsr(msr: u32) -> u64 {
    unsafe {
        let eax: u32;
        let edx: u32;
        asm!("rdmsr", out("eax") eax, out("edx") edx, in("ecx") msr);
        return (eax as u64) | ((edx as u64) << 32);
    }
}

#[inline]
pub fn rdtsc() -> u64 {
    unsafe {
        let eax: u32;
        let edx: u32;
        asm!("rdtsc", out("eax") eax, out("edx") edx);
        return (eax as u64) | ((edx as u64) << 32);
    }
}

#[inline]
pub fn fxsave(memory: *mut u8) {
    unsafe {
        asm! ("fxsave [{0}]", in(reg) memory);
    }
}

#[inline]
pub fn fxrstor(memory: *const u8) {
    unsafe {
        asm! ("fxrstor [{0}]", in(reg) memory);
    }
}

#[inline]
pub fn xsave(memory: *mut u8) {
    unsafe {
        asm! ("xsave [{0}]", in(reg) memory);
    }
}

#[inline]
pub fn xrstor(memory: *const u8) {
    unsafe {
        asm! ("xrstor [{0}]", in(reg) memory);
    }
}

#[inline]
pub fn read8(port: u16) -> u8 {
    unsafe {
        let value: u8;
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
        return value;
    }
}

#[inline]
pub fn read16(port: u16) -> u16 {
    unsafe {
        let value: u16;
        asm!("in ax, dx", out("ax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        return value;
    }
}

#[inline]
pub fn read32(port: u16) -> u32 {
    unsafe {
        let value: u32;
        asm!("in eax, dx", out("eax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        return value;
    }
}

#[inline]
pub fn write8(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
pub fn write16(port: u16, value: u16) {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
pub fn write32(port: u16, value: u32) {
    unsafe {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
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
