// x86 specific declarations.

#pragma once

#include <menix/common.h>

#include <gdt.h>
#include <tss.h>

// CPUID Leaf 1 ECX
#define CPUID_1C_SSE3		(1 << 0)
#define CPUID_1C_PCLMUL		(1 << 1)
#define CPUID_1C_DTES64		(1 << 2)
#define CPUID_1C_MONITOR	(1 << 3)
#define CPUID_1C_DS_CPL		(1 << 4)
#define CPUID_1C_VMX		(1 << 5)
#define CPUID_1C_SMX		(1 << 6)
#define CPUID_1C_EST		(1 << 7)
#define CPUID_1C_TM2		(1 << 8)
#define CPUID_1C_SSSE3		(1 << 9)
#define CPUID_1C_CID		(1 << 10)
#define CPUID_1C_SDBG		(1 << 11)
#define CPUID_1C_FMA		(1 << 12)
#define CPUID_1C_CX16		(1 << 13)
#define CPUID_1C_XTPR		(1 << 14)
#define CPUID_1C_PDCM		(1 << 15)
#define CPUID_1C_PCID		(1 << 17)
#define CPUID_1C_DCA		(1 << 18)
#define CPUID_1C_SSE4_1		(1 << 19)
#define CPUID_1C_SSE4_2		(1 << 20)
#define CPUID_1C_X2APIC		(1 << 21)
#define CPUID_1C_MOVBE		(1 << 22)
#define CPUID_1C_POPCNT		(1 << 23)
#define CPUID_1C_TSC		(1 << 24)
#define CPUID_1C_AES		(1 << 25)
#define CPUID_1C_XSAVE		(1 << 26)
#define CPUID_1C_OSXSAVE	(1 << 27)
#define CPUID_1C_AVX		(1 << 28)
#define CPUID_1C_F16C		(1 << 29)
#define CPUID_1C_RDRAND		(1 << 30)
#define CPUID_1C_HYPERVISOR (1 << 31)

// CPUID Leaf 1 EDX
#define CPUID_1D_FPU	 (1 << 0)
#define CPUID_1D_VME	 (1 << 1)
#define CPUID_1D_DE		 (1 << 2)
#define CPUID_1D_PSE	 (1 << 3)
#define CPUID_1D_TSC	 (1 << 4)
#define CPUID_1D_MSR	 (1 << 5)
#define CPUID_1D_PAE	 (1 << 6)
#define CPUID_1D_MCE	 (1 << 7)
#define CPUID_1D_CX8	 (1 << 8)
#define CPUID_1D_APIC	 (1 << 9)
#define CPUID_1D_SEP	 (1 << 11)
#define CPUID_1D_MTRR	 (1 << 12)
#define CPUID_1D_PGE	 (1 << 13)
#define CPUID_1D_MCA	 (1 << 14)
#define CPUID_1D_CMOV	 (1 << 15)
#define CPUID_1D_PAT	 (1 << 16)
#define CPUID_1D_PSE36	 (1 << 17)
#define CPUID_1D_PSN	 (1 << 18)
#define CPUID_1D_CLFLUSH (1 << 19)
#define CPUID_1D_DS		 (1 << 21)
#define CPUID_1D_ACPI	 (1 << 22)
#define CPUID_1D_MMX	 (1 << 23)
#define CPUID_1D_FXSR	 (1 << 24)
#define CPUID_1D_SSE	 (1 << 25)
#define CPUID_1D_SSE2	 (1 << 26)
#define CPUID_1D_SS		 (1 << 27)
#define CPUID_1D_HTT	 (1 << 28)
#define CPUID_1D_TM		 (1 << 29)
#define CPUID_1D_IA64	 (1 << 30)
#define CPUID_1D_PBE	 (1 << 31)

// CPUID Leaf 7 EBX
#define CPUID_7B_FSGSBASE	(1 << 0)
#define CPUID_7B_SGX		(1 << 2)
#define CPUID_7B_BMI		(1 << 3)
#define CPUID_7B_HLE		(1 << 4)
#define CPUID_7B_AVX2		(1 << 5)
#define CPUID_7B_SMEP		(1 << 7)
#define CPUID_7B_BMI2		(1 << 8)
#define CPUID_7B_RTM		(1 << 11)
#define CPUID_7B_AVX512F	(1 << 16)
#define CPUID_7B_AVX512DQ	(1 << 17)
#define CPUID_7B_RDSEED		(1 << 18)
#define CPUID_7B_ADX		(1 << 19)
#define CPUID_7B_SMAP		(1 << 20)
#define CPUID_7B_AVX512IFMA (1 << 21)
#define CPUID_7B_CLFLUSHOPT (1 << 23)
#define CPUID_7B_CLWB		(1 << 24)
#define CPUID_7B_AVX512CD	(1 << 28)
#define CPUID_7B_SHA		(1 << 29)
#define CPUID_7B_AVX512BW	(1 << 30)
#define CPUID_7B_AVX512VL	(1u << 31)

// CPUID Leaf 7 ECX
#define CPUID_7C_AVX512VBMI		 (1 << 1)
#define CPUID_7C_UMIP			 (1 << 2)
#define CPUID_7C_PKU			 (1 << 3)
#define CPUID_7C_OSPKE			 (1 << 4)
#define CPUID_7C_WAITPKG		 (1 << 5)
#define CPUID_7C_AVX512VBMI2	 (1 << 6)
#define CPUID_7C_SHSTK			 (1 << 7)
#define CPUID_7C_GFNI			 (1 << 8)
#define CPUID_7C_VAES			 (1 << 9)
#define CPUID_7C_VPCLMULQDQ		 (1 << 10)
#define CPUID_7C_AVX512VNNI		 (1 << 11)
#define CPUID_7C_AVX512BITALG	 (1 << 12)
#define CPUID_7C_TME_EN			 (1 << 13)
#define CPUID_7C_AVX512VPOPCNTDQ (1 << 14)
#define CPUID_7C_RDPID			 (1 << 22)
#define CPUID_7C_KL				 (1 << 23)
#define CPUID_7C_CLDEMOTE		 (1 << 25)
#define CPUID_7C_MOVDIRI		 (1 << 27)
#define CPUID_7C_MOVDIR64B		 (1 << 28)
#define CPUID_7C_ENQCMD			 (1 << 29)

// CPUID Leaf 7 EDX
#define CPUID_7D_UINTR				(1 << 5)
#define CPUID_7D_AVX512VP2INTERSECT (1 << 8)
#define CPUID_7D_SERIALIZE			(1 << 14)
#define CPUID_7D_TSXLDTRK			(1 << 16)
#define CPUID_7D_PCONFIG			(1 << 18)
#define CPUID_7D_IBT				(1 << 20)
#define CPUID_7D_AMX_BF16			(1 << 22)
#define CPUID_7D_AVX512FP16			(1 << 23)
#define CPUID_7D_AMX_TILE			(1 << 24)
#define CPUID_7D_AMX_INT8			(1 << 25)

#define RFLAGS_CF	(1 << 0)	 // Carry floating
#define RFLAGS_PF	(1 << 2)	 // Parity Flag
#define RFLAGS_AF	(1 << 4)	 // Auxiliary Carry Flag
#define RFLAGS_ZF	(1 << 6)	 // Zero Flag
#define RFLAGS_SF	(1 << 7)	 // Sign Flag
#define RFLAGS_TF	(1 << 8)	 // Trap Flag
#define RFLAGS_IF	(1 << 9)	 // Interrupt Enable Flag
#define RFLAGS_DF	(1 << 10)	 // Direction Flag
#define RFLAGS_OF	(1 << 11)	 // Overflow Flag
#define RFLAGS_IOPL (3 << 12)	 // I/O Privilege Level
#define RFLAGS_NT	(1 << 14)	 // Nested Task
#define RFLAGS_RF	(1 << 16)	 // Resume Flag
#define RFLAGS_VM	(1 << 17)	 // Virtual-8086 Mode
#define RFLAGS_AC	(1 << 18)	 // Alignment Check / Access Control
#define RFLAGS_VIF	(1 << 19)	 // Virtual Interrupt Flag
#define RFLAGS_VIP	(1 << 20)	 // Virtual Interrupt Pending
#define RFLAGS_ID	(1 << 21)	 // ID Flag

#define CR0_PE (1 << 0)		// Protected Mode Enable
#define CR0_MP (1 << 1)		// Monitor Co-Processor
#define CR0_EM (1 << 2)		// Emulation
#define CR0_TS (1 << 3)		// Task Switched
#define CR0_ET (1 << 4)		// Extension Type
#define CR0_NE (1 << 5)		// Numeric Error
#define CR0_WP (1 << 16)	// Write Protect
#define CR0_AM (1 << 18)	// Alignment Mask
#define CR0_NW (1 << 29)	// Not-Write Through
#define CR0_CD (1 << 30)	// Cache Disable
#define CR0_PG (1 << 31)	// Paging

#define CR4_VME		   (1 << 0)		// Virtual-8086 Mode Extensions
#define CR4_PVI		   (1 << 1)		// Protected Mode Virtual Interrupts
#define CR4_TSD		   (1 << 2)		// Time Stamp enabled only in ring 0
#define CR4_DE		   (1 << 3)		// Debugging Extensions
#define CR4_PSE		   (1 << 4)		// Page Size Extension
#define CR4_PAE		   (1 << 5)		// Physical Address Extension
#define CR4_MCE		   (1 << 6)		// Machine Check Exception
#define CR4_PGE		   (1 << 7)		// Page Global Enable
#define CR4_PCE		   (1 << 8)		// Performance Monitoring Counter Enable
#define CR4_OSFXSR	   (1 << 9)		// OS support for fxsave and fxrstor instructions
#define CR4_OSXMMEXCPT (1 << 10)	// OS Support for unmasked simd floating point exceptions
#define CR4_UMIP	   (1 << 11)	// User-Mode Instruction Prevention
#define CR4_VMXE	   (1 << 13)	// Virtual Machine Extensions Enable
#define CR4_SMXE	   (1 << 14)	// Safer Mode Extensions Enable
#define CR4_FSGSBASE   (1 << 16)	// Enables the instructions RDFSBASE, RDGSBASE, WRFSBASE, and WRGSBASE
#define CR4_PCIDE	   (1 << 17)	// PCID Enable
#define CR4_OSXSAVE	   (1 << 18)	// XSAVE And Processor Extended States Enable
#define CR4_SMEP	   (1 << 20)	// Supervisor Mode Executions Protection Enable
#define CR4_SMAP	   (1 << 21)	// Supervisor Mode Access Protection Enable
#define CR4_PKE		   (1 << 22)	// Enable protection keys for user-mode pages
#define CR4_CET		   (1 << 23)	// Enable Control-flow Enforcement Technology
#define CR4_PKS		   (1 << 24)	// Enable protection keys for supervisor-mode pages

#define MSR_EFER		   0xC0000080
#define MSR_EFER_SCE	   (1 << 0)		// System Call Extensions
#define MSR_EFER_LME	   (1 << 8)		// Long Mode Enable
#define MSR_EFER_LMA	   (1 << 10)	// Long Mode Active
#define MSR_EFER_NXE	   (1 << 11)	// No-Execute Enable
#define MSR_EFER_SVME	   (1 << 12)	// Secure Virtual Machine Enable
#define MSR_EFER_LMSLE	   (1 << 13)	// Long Mode Segment Limit Enable
#define MSR_EFER_FFXSR	   (1 << 14)	// Fast FXSAVE/FXRSTOR
#define MSR_EFER_TCE	   (1 << 15)	// Translation Cache Extension
#define MSR_STAR		   0xC0000081
#define MSR_LSTAR		   0xC0000082
#define MSR_CSTAR		   0xC0000083
#define MSR_SFMASK		   0xC0000084
#define MSR_FS_BASE		   0xC0000100
#define MSR_GS_BASE		   0xC0000101
#define MSR_KERNEL_GS_BASE 0xC0000102

#define CPL_USER   (0b11)
#define CPL_KERNEL (0b00)

#define INT_TIMER	0x20
#define INT_SYSCALL 0x80

typedef struct
{
	u16 fcw;
	u16 fsw;
	u8 ftw;
	u8 reserved0;
	u16 fop;
	u64 fpuIp;
	u64 fpuDp;
	u32 mxcsr;
	u32 mxcsrMask;
	u8 st0[10];
	u8 reserved1[6];
	u8 st1[10];
	u8 reserved2[6];
	u8 st2[10];
	u8 reserved3[6];
	u8 st3[10];
	u8 reserved4[6];
	u8 st4[10];
	u8 reserved5[6];
	u8 st5[10];
	u8 reserved6[6];
	u8 st6[10];
	u8 reserved7[6];
	u8 st7[10];
	u8 reserved8[6];
	u8 xmm0[16];
	u8 xmm1[16];
	u8 xmm2[16];
	u8 xmm3[16];
	u8 xmm4[16];
	u8 xmm5[16];
	u8 xmm6[16];
	u8 xmm7[16];
	u8 xmm8[16];
	u8 xmm9[16];
	u8 xmm10[16];
	u8 xmm11[16];
	u8 xmm12[16];
	u8 xmm13[16];
	u8 xmm14[16];
	u8 xmm15[16];
	u8 reserved9[48];
	u8 available[48];
} FxState;
