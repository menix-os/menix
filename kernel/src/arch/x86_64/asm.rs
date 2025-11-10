#![allow(unused)]

use core::arch::{asm, global_asm};

use crate::{
    memory::pmm::{AllocFlags, KernelAlloc, PageAllocator},
    posix::errno::EResult,
};

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
        asm!("fxsave [{0}]", in(reg) memory);
    }
}

#[inline]
pub unsafe fn fxrstor(memory: *const u8) {
    unsafe {
        asm!("fxrstor [{0}]", in(reg) memory);
    }
}

#[inline]
pub unsafe fn xsave(memory: *mut u8) {
    unsafe {
        asm!("xsave [{0}]", in(reg) memory, in("eax")0xffff_ffffu32, in("edx")0xffff_ffffu32);
    }
}

#[inline]
pub unsafe fn xrstor(memory: *const u8) {
    unsafe {
        asm!("xrstor [{0}]", in(reg) memory, in("eax")0xffff_ffffu32, in("edx")0xffff_ffffu32);
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

#[inline]
pub unsafe fn read_ds() -> u16 {
    unsafe {
        let mut value = 0;
        asm!("mov {0:x}, ds", out(reg) value);
        return value;
    }
}

#[inline]
pub unsafe fn read_es() -> u16 {
    unsafe {
        let mut value = 0;
        asm!("mov {0:x}, es", out(reg) value);
        return value;
    }
}

#[inline]
pub unsafe fn read_fs() -> u16 {
    unsafe {
        let mut value = 0;
        asm!("mov {0:x}, fs", out(reg) value);
        return value;
    }
}

#[inline]
pub unsafe fn read_gs() -> u16 {
    unsafe {
        let mut value = 0;
        asm!("mov {0:x}, gs", out(reg) value);
        return value;
    }
}

#[inline]
pub unsafe fn write_ds(value: u16) {
    unsafe {
        asm!("mov ds, {0:x}", in(reg) value);
    }
}

#[inline]
pub unsafe fn write_es(value: u16) {
    unsafe {
        asm!("mov es, {0:x}", in(reg) value);
    }
}

#[inline]
pub unsafe fn write_fs(value: u16) {
    unsafe {
        asm!("mov fs, {0:x}", in(reg) value);
    }
}

#[inline]
pub unsafe fn write_gs(value: u16) {
    unsafe {
        asm!("mov gs, {0:x}", in(reg) value);
    }
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct FxState {
    pub fcw: u16,
    pub fsw: u16,
    pub ftw: u8,
    reserved0: u8,
    pub fop: u16,
    pub fpu_ip: u64,
    pub fpu_dp: u64,
    pub mxcsr: u32,
    pub mxcsr_mask: u32,
    pub st0: [u8; 10],
    reserved1: [u8; 6],
    pub st1: [u8; 10],
    reserved2: [u8; 6],
    pub st2: [u8; 10],
    reserved3: [u8; 6],
    pub st3: [u8; 10],
    reserved4: [u8; 6],
    pub st4: [u8; 10],
    reserved5: [u8; 6],
    pub st5: [u8; 10],
    reserved6: [u8; 6],
    pub st6: [u8; 10],
    reserved7: [u8; 6],
    pub st7: [u8; 10],
    reserved8: [u8; 6],
    pub xmm0: [u8; 16],
    pub xmm1: [u8; 16],
    pub xmm2: [u8; 16],
    pub xmm3: [u8; 16],
    pub xmm4: [u8; 16],
    pub xmm5: [u8; 16],
    pub xmm6: [u8; 16],
    pub xmm7: [u8; 16],
    pub xmm8: [u8; 16],
    pub xmm9: [u8; 16],
    pub xmm10: [u8; 16],
    pub xmm11: [u8; 16],
    pub xmm12: [u8; 16],
    pub xmm13: [u8; 16],
    pub xmm14: [u8; 16],
    pub xmm15: [u8; 16],
    reserved9: [u8; 48],
    pub available: [u8; 48],
    pub xstate_bv: u64,
    pub xcomp_bv: u64,
}
