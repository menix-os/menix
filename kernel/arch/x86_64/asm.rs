use super::gdt::GdtRegister;
use super::idt::IdtRegister;

/// Wrapper for the `lgdt` instruction.
/// Only changing the GDT on its own is technically unsafe.
pub unsafe fn lgdt(gdt: &GdtRegister) {
    core::arch::asm!("lgdt [{0}]", in(reg) gdt);
}

/// Wrapper for the `lidt` instruction.
/// Only changing the IDT on its own is technically unsafe.
pub unsafe fn lidt(idt: &IdtRegister) {
    core::arch::asm!("lidt [{0}]", in(reg) idt);
}

/// Wrapper for the `cpuid` instruction.
pub unsafe fn cpuid(a: &mut usize, b: &mut usize, c: &mut usize, d: &mut usize) {
    let result = core::arch::x86_64::__cpuid_count(*c as u32, *a as u32);
    *a = result.eax as usize;
    *b = result.ebx as usize;
    *c = result.ecx as usize;
    *d = result.edx as usize;
}

// CPUID Leaf 1 ECX
pub const CPUID_1C_SSE3: usize = 1 << 0;
pub const CPUID_1C_PCLMUL: usize = 1 << 1;
pub const CPUID_1C_DTES64: usize = 1 << 2;
pub const CPUID_1C_MONITOR: usize = 1 << 3;
pub const CPUID_1C_DS_CPL: usize = 1 << 4;
pub const CPUID_1C_VMX: usize = 1 << 5;
pub const CPUID_1C_SMX: usize = 1 << 6;
pub const CPUID_1C_EST: usize = 1 << 7;
pub const CPUID_1C_TM2: usize = 1 << 8;
pub const CPUID_1C_SSSE3: usize = 1 << 9;
pub const CPUID_1C_CID: usize = 1 << 10;
pub const CPUID_1C_SDBG: usize = 1 << 11;
pub const CPUID_1C_FMA: usize = 1 << 12;
pub const CPUID_1C_CX16: usize = 1 << 13;
pub const CPUID_1C_XTPR: usize = 1 << 14;
pub const CPUID_1C_PDCM: usize = 1 << 15;
pub const CPUID_1C_PCID: usize = 1 << 17;
pub const CPUID_1C_DCA: usize = 1 << 18;
pub const CPUID_1C_SSE4_1: usize = 1 << 19;
pub const CPUID_1C_SSE4_2: usize = 1 << 20;
pub const CPUID_1C_X2APIC: usize = 1 << 21;
pub const CPUID_1C_MOVBE: usize = 1 << 22;
pub const CPUID_1C_POPCNT: usize = 1 << 23;
pub const CPUID_1C_TSC: usize = 1 << 24;
pub const CPUID_1C_AES: usize = 1 << 25;
pub const CPUID_1C_XSAVE: usize = 1 << 26;
pub const CPUID_1C_OSXSAVE: usize = 1 << 27;
pub const CPUID_1C_AVX: usize = 1 << 28;
pub const CPUID_1C_F16C: usize = 1 << 29;
pub const CPUID_1C_RDRAND: usize = 1 << 30;
pub const CPUID_1C_HYPERVISOR: usize = 1 << 31;

// CPUID Leaf 1 EDX
pub const CPUID_1D_FPU: usize = 1 << 0;
pub const CPUID_1D_VME: usize = 1 << 1;
pub const CPUID_1D_DE: usize = 1 << 2;
pub const CPUID_1D_PSE: usize = 1 << 3;
pub const CPUID_1D_TSC: usize = 1 << 4;
pub const CPUID_1D_MSR: usize = 1 << 5;
pub const CPUID_1D_PAE: usize = 1 << 6;
pub const CPUID_1D_MCE: usize = 1 << 7;
pub const CPUID_1D_CX8: usize = 1 << 8;
pub const CPUID_1D_APIC: usize = 1 << 9;
pub const CPUID_1D_SEP: usize = 1 << 11;
pub const CPUID_1D_MTRR: usize = 1 << 12;
pub const CPUID_1D_PGE: usize = 1 << 13;
pub const CPUID_1D_MCA: usize = 1 << 14;
pub const CPUID_1D_CMOV: usize = 1 << 15;
pub const CPUID_1D_PAT: usize = 1 << 16;
pub const CPUID_1D_PSE36: usize = 1 << 17;
pub const CPUID_1D_PSN: usize = 1 << 18;
pub const CPUID_1D_CLFLUSH: usize = 1 << 19;
pub const CPUID_1D_DS: usize = 1 << 21;
pub const CPUID_1D_ACPI: usize = 1 << 22;
pub const CPUID_1D_MMX: usize = 1 << 23;
pub const CPUID_1D_FXSR: usize = 1 << 24;
pub const CPUID_1D_SSE: usize = 1 << 25;
pub const CPUID_1D_SSE2: usize = 1 << 26;
pub const CPUID_1D_SS: usize = 1 << 27;
pub const CPUID_1D_HTT: usize = 1 << 28;
pub const CPUID_1D_TM: usize = 1 << 29;
pub const CPUID_1D_IA64: usize = 1 << 30;
pub const CPUID_1D_PBE: usize = 1 << 31;

// CPUID Leaf 7 EBX
pub const CPUID_7B_FSGSBASE: usize = 1 << 0;
pub const CPUID_7B_SGX: usize = 1 << 2;
pub const CPUID_7B_BMI: usize = 1 << 3;
pub const CPUID_7B_HLE: usize = 1 << 4;
pub const CPUID_7B_AVX2: usize = 1 << 5;
pub const CPUID_7B_SMEP: usize = 1 << 7;
pub const CPUID_7B_BMI2: usize = 1 << 8;
pub const CPUID_7B_RTM: usize = 1 << 11;
pub const CPUID_7B_AVX512F: usize = 1 << 16;
pub const CPUID_7B_AVX512DQ: usize = 1 << 17;
pub const CPUID_7B_RDSEED: usize = 1 << 18;
pub const CPUID_7B_ADX: usize = 1 << 19;
pub const CPUID_7B_SMAP: usize = 1 << 20;
pub const CPUID_7B_AVX512IFMA: usize = 1 << 21;
pub const CPUID_7B_CLFLUSHOPT: usize = 1 << 23;
pub const CPUID_7B_CLWB: usize = 1 << 24;
pub const CPUID_7B_AVX512CD: usize = 1 << 28;
pub const CPUID_7B_SHA: usize = 1 << 29;
pub const CPUID_7B_AVX512BW: usize = 1 << 30;
pub const CPUID_7B_AVX512VL: usize = 1 << 31;

// CPUID Leaf 7 ECX
pub const CPUID_7C_AVX512VBMI: usize = 1 << 1;
pub const CPUID_7C_UMIP: usize = 1 << 2;
pub const CPUID_7C_PKU: usize = 1 << 3;
pub const CPUID_7C_OSPKE: usize = 1 << 4;
pub const CPUID_7C_WAITPKG: usize = 1 << 5;
pub const CPUID_7C_AVX512VBMI2: usize = 1 << 6;
pub const CPUID_7C_SHSTK: usize = 1 << 7;
pub const CPUID_7C_GFNI: usize = 1 << 8;
pub const CPUID_7C_VAES: usize = 1 << 9;
pub const CPUID_7C_VPCLMULQDQ: usize = 1 << 10;
pub const CPUID_7C_AVX512VNNI: usize = 1 << 11;
pub const CPUID_7C_AVX512BITALG: usize = 1 << 12;
pub const CPUID_7C_TME_EN: usize = 1 << 13;
pub const CPUID_7C_AVX512VPOPCNTDQ: usize = 1 << 14;
pub const CPUID_7C_RDPID: usize = 1 << 22;
pub const CPUID_7C_KL: usize = 1 << 23;
pub const CPUID_7C_CLDEMOTE: usize = 1 << 25;
pub const CPUID_7C_MOVDIRI: usize = 1 << 27;
pub const CPUID_7C_MOVDIR64B: usize = 1 << 28;
pub const CPUID_7C_ENQCMD: usize = 1 << 29;

// CPUID Leaf 7 EDX
pub const CPUID_7D_UINTR: usize = 1 << 5;
pub const CPUID_7D_AVX512VP2INTERSECT: usize = 1 << 8;
pub const CPUID_7D_SERIALIZE: usize = 1 << 14;
pub const CPUID_7D_TSXLDTRK: usize = 1 << 16;
pub const CPUID_7D_PCONFIG: usize = 1 << 18;
pub const CPUID_7D_IBT: usize = 1 << 20;
pub const CPUID_7D_AMX_BF16: usize = 1 << 22;
pub const CPUID_7D_AVX512FP16: usize = 1 << 23;
pub const CPUID_7D_AMX_TILE: usize = 1 << 24;
pub const CPUID_7D_AMX_INT8: usize = 1 << 25;
