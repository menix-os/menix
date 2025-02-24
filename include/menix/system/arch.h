// Architecture dependent platform init and deinit

#pragma once

#include <menix/system/boot.h>
#include <menix/system/interrupts.h>

#include <bits/arch.h>
#include <bits/asm.h>
#include <bits/context.h>

#define arch_log(fmt, ...) print_log(MENIX_ARCH ": " fmt, ##__VA_ARGS__)

// The size of a single page.
#ifdef ARCH_HAS_DYNAMIC_PAGE_SIZE
extern usize arch_page_size;
#else
#define arch_page_size ((usize)(0x1000))
#endif

// CPU-local information.
typedef struct [[gnu::aligned(arch_page_size)]] Cpu
{
	usize id;						  // Unique ID of this CPU.
	usize kernel_stack;				  // Stack pointer for the kernel.
	usize user_stack;				  // Stack pointer for the user space.
	struct Thread* thread;			  // Current thread running on this CPU.
	usize ticks_active;				  // The amount of ticks the running thread has been active.
	bool is_present;				  // If the CPU is present.
	InterruptFn irq_handlers[256];	  // IRQ handlers.
	void* irq_data[256];			  // IRQ context to pass along.

#ifdef __x86_64__
	Gdt gdt;
	TaskStateSegment tss;
	u32 lapic_id;					   // Local APIC ID.
	usize fpu_size;					   // Size of the FPU in bytes.
	void (*fpu_save)(void* dst);	   // Function to call when saving the FPU state.
	void (*fpu_restore)(void* dst);	   // Function to call when restoring the FPU state.
#elif defined(__riscv) && (__riscv_xlen == 64)
	u32 hart_id;	// Hart CPU ID.
#endif
} CpuInfo;

#define MAX_CPUS 1024

extern CpuInfo per_cpu_data[MAX_CPUS];

// Initializes the platform for use by the kernel and boot routines.
void arch_early_init();

// Initializes the rest of the platform after the boot routines have completed.
void arch_init(BootInfo* info);

// Initializes a single processor.
// `cpu`: Information about the CPU that has to be enabled.
void arch_init_cpu(CpuInfo* cpu, CpuInfo* boot_cpu);

// Disables a single processor.
bool arch_stop_cpu(usize id);

// Halts all CPUs.
[[noreturn]] void arch_stop();

// Writes all registers to the current output stream.
void arch_dump_registers(Context* regs);

// Gets processor metadata.
CpuInfo* arch_current_cpu();

typedef enum : usize
{
	ArchCtl_None = 0,

#ifdef __x86_64__
	ArchCtl_SetFsBase = 1,
#endif
} ArchCtl;

usize arch_archctl(ArchCtl ctl, usize arg1, usize arg2);
