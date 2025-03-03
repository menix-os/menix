#![allow(unused)]

/// Carry floating
pub const RFLAGS_CF: usize = 1 << 0;
/// Parity Flag
pub const RFLAGS_PF: usize = 1 << 2;
/// Auxiliary Carry Flag
pub const RFLAGS_AF: usize = 1 << 4;
/// Zero Flag
pub const RFLAGS_ZF: usize = 1 << 6;
/// Sign Flag
pub const RFLAGS_SF: usize = 1 << 7;
/// Trap Flag
pub const RFLAGS_TF: usize = 1 << 8;
/// Interrupt Enable Flag
pub const RFLAGS_IF: usize = 1 << 9;
/// Direction Flag
pub const RFLAGS_DF: usize = 1 << 10;
/// Overflow Flag
pub const RFLAGS_OF: usize = 1 << 11;
/// I/O Privilege Level
pub const RFLAGS_IOPL: usize = 3 << 12;
/// Nested Task
pub const RFLAGS_NT: usize = 1 << 14;
/// Resume Flag
pub const RFLAGS_RF: usize = 1 << 16;
/// Virtual-8086 Mode
pub const RFLAGS_VM: usize = 1 << 17;
/// Alignment Check / Access Control
pub const RFLAGS_AC: usize = 1 << 18;
/// Virtual Interrupt Flag
pub const RFLAGS_VIF: usize = 1 << 19;
/// Virtual Interrupt Pending
pub const RFLAGS_VIP: usize = 1 << 20;
/// ID Flag
pub const RFLAGS_ID: usize = 1 << 21;

// Control register 0
/// Protected Mode Enable
pub const CR0_PE: usize = 1 << 0;
/// Monitor Co-Processor
pub const CR0_MP: usize = 1 << 1;
/// Emulation
pub const CR0_EM: usize = 1 << 2;
/// Task Switched
pub const CR0_TS: usize = 1 << 3;
/// Extension Type
pub const CR0_ET: usize = 1 << 4;
/// Numeric Error
pub const CR0_NE: usize = 1 << 5;
/// Write Protect
pub const CR0_WP: usize = 1 << 16;
/// Alignment Mask
pub const CR0_AM: usize = 1 << 18;
/// Not-Write Through
pub const CR0_NW: usize = 1 << 29;
/// Cache Disable
pub const CR0_CD: usize = 1 << 30;
/// Paging
pub const CR0_PG: usize = 1 << 31;

// Control register 4
/// Virtual-8086 Mode Extensions
pub const CR4_VME: usize = 1 << 0;
/// Protected Mode Virtual Interrupts
pub const CR4_PVI: usize = 1 << 1;
/// Time Stamp enabled only in ring 0
pub const CR4_TSD: usize = 1 << 2;
/// Debugging Extensions
pub const CR4_DE: usize = 1 << 3;
/// Page Size Extension
pub const CR4_PSE: usize = 1 << 4;
/// Physical Address Extension
pub const CR4_PAE: usize = 1 << 5;
/// Machine Check Exception
pub const CR4_MCE: usize = 1 << 6;
/// Page Global Enable
pub const CR4_PGE: usize = 1 << 7;
/// Performance Monitoring Counter Enable
pub const CR4_PCE: usize = 1 << 8;
/// OS support for fxsave and fxrstor instructions
pub const CR4_OSFXSR: usize = 1 << 9;
/// OS Support for unmasked simd floating point exceptions
pub const CR4_OSXMMEXCPT: usize = 1 << 10;
/// User-Mode Instruction Prevention
pub const CR4_UMIP: usize = 1 << 11;
/// Virtual Machine Extensions Enable
pub const CR4_VMXE: usize = 1 << 13;
/// Safer Mode Extensions Enable
pub const CR4_SMXE: usize = 1 << 14;
/// Enables the instructions RDFSBASE, RDGSBASE, WRFSBASE, and WRGSBASE
pub const CR4_FSGSBASE: usize = 1 << 16;
/// PCID Enable
pub const CR4_PCIDE: usize = 1 << 17;
/// XSAVE And Processor Extended States Enable
pub const CR4_OSXSAVE: usize = 1 << 18;
/// Supervisor Mode Executions Protection Enable
pub const CR4_SMEP: usize = 1 << 20;
/// Supervisor Mode Access Protection Enable
pub const CR4_SMAP: usize = 1 << 21;
/// Enable protection keys for user-mode pages
pub const CR4_PKE: usize = 1 << 22;
/// Enable Control-flow Enforcement Technology
pub const CR4_CET: usize = 1 << 23;
/// Enable protection keys for supervisor-mode pages
pub const CR4_PKS: usize = 1 << 24;

// Model specific registers
pub const MSR_EFER: u32 = 0xC0000080;
/// System Call Extensions
pub const MSR_EFER_SCE: u32 = 1 << 0;
/// Long Mode Enable
pub const MSR_EFER_LME: u32 = 1 << 8;
/// Long Mode Active
pub const MSR_EFER_LMA: u32 = 1 << 10;
/// No-Execute Enable
pub const MSR_EFER_NXE: u32 = 1 << 11;
/// Secure Virtual Machine Enable
pub const MSR_EFER_SVME: u32 = 1 << 12;
/// Long Mode Segment Limit Enable
pub const MSR_EFER_LMSLE: u32 = 1 << 13;
/// Fast FXSAVE/FXRSTOR
pub const MSR_EFER_FFXSR: u32 = 1 << 14;
/// Translation Cache Extension
pub const MSR_EFER_TCE: u32 = 1 << 15;
pub const MSR_STAR: u32 = 0xC0000081;
pub const MSR_LSTAR: u32 = 0xC0000082;
pub const MSR_CSTAR: u32 = 0xC0000083;
pub const MSR_SFMASK: u32 = 0xC0000084;
pub const MSR_FS_BASE: u32 = 0xC0000100;
pub const MSR_GS_BASE: u32 = 0xC0000101;
pub const MSR_KERNEL_GS_BASE: u32 = 0xC0000102;

pub const CPL_USER: u8 = 0b11;
pub const CPL_KERNEL: u8 = 0b00;

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
