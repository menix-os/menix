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
