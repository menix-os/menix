#![allow(unused)]

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
    let eax;
    let ebx;
    let ecx;
    let edx;

    // LLVM sometimes reserves `ebx` for its internal use, we so we need to use
    // a scratch register for it instead.
    unsafe {
        asm!(
            "mov {0:r}, rbx",
            "cpuid",
            "xchg {0:r}, rbx",
            out(reg) ebx,
            inout("eax") leaf => eax,
            inout("ecx") sub_leaf => ecx,
            out("edx") edx,
            options(nostack, preserves_flags),
        );
    }

    CpuIdResult { eax, ebx, ecx, edx }
}

/// Writes an unsigned 64-bit value to a model-specific register.
#[inline]
pub unsafe fn wrmsr(msr: u32, value: u64) {
    unsafe {
        let eax = value as u32;
        let edx = (value >> 32) as u32;
        asm!("wrmsr", in("eax") eax, in("edx") edx, in("ecx") msr);
    }
}

/// Writes an unsigned 64-bit value to the model-specific XCR register.
#[inline]
pub unsafe fn wrxcr(msr: u32, value: u64) {
    unsafe {
        let eax = value as u32;
        let edx = (value >> 32) as u32;
        asm!("xsetbv", in("eax") eax, in("edx") edx, in("ecx") msr);
    }
}

/// Reads an unsigned 64-bit value from a model-specific register.
#[inline]
pub unsafe fn rdmsr(msr: u32) -> u64 {
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
pub unsafe fn fxsave(memory: *mut u8) {
    unsafe {
        asm! ("fxsave [{0}]", in(reg) memory);
    }
}

#[inline]
pub unsafe fn fxrstor(memory: *const u8) {
    unsafe {
        asm! ("fxrstor [{0}]", in(reg) memory);
    }
}

#[inline]
pub unsafe fn xsave(memory: *mut u8) {
    unsafe {
        asm! ("xsave [{0}]", in(reg) memory);
    }
}

#[inline]
pub unsafe fn xrstor(memory: *const u8) {
    unsafe {
        asm! ("xrstor [{0}]", in(reg) memory);
    }
}

#[inline]
pub unsafe fn read8(port: u16) -> u8 {
    unsafe {
        let value: u8;
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
        return value;
    }
}

#[inline]
pub unsafe fn read16(port: u16) -> u16 {
    unsafe {
        let value: u16;
        asm!("in ax, dx", out("ax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        return value;
    }
}

#[inline]
pub unsafe fn read32(port: u16) -> u32 {
    unsafe {
        let value: u32;
        asm!("in eax, dx", out("eax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        return value;
    }
}

#[inline]
pub unsafe fn write8(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
pub unsafe fn write16(port: u16, value: u16) {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
pub unsafe fn write32(port: u16, value: u32) {
    unsafe {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}
