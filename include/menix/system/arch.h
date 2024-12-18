// Architecture dependent platform init and deinit

#pragma once

#include <menix/system/boot.h>
#include <menix/system/interrupts.h>

#include <bits/arch.h>
#include <bits/asm.h>
#include <bits/context.h>

#define arch_log(fmt, ...) print_log(CONFIG_arch ": " fmt, ##__VA_ARGS__)

// The size of a single page.
#ifdef CONFIG_dynamic_page_size
extern usize arch_page_size;
#else
#define arch_page_size ((usize)(0x1000))
#endif

// CPU-local information.
typedef struct Cpu
{
	usize id;						  // Unique ID of this CPU.
	usize kernel_stack;				  // Stack pointer for the kernel.
	usize user_stack;				  // Stack pointer for the user space.
	struct Thread* thread;			  // Current thread running on this CPU.
	usize ticks_active;				  // The amount of ticks the running thread has been active.
	bool is_present;				  // If the CPU is present.
	InterruptFn irq_handlers[256];	  // IRQ handlers.
	InterruptFn irq_data[256];		  // IRQ context to pass along.

#ifdef CONFIG_arch_x86_64
	Gdt gdt;
	TaskStateSegment tss;
	u32 lapic_id;					   // Local APIC ID.
	usize fpu_size;					   // Size of the FPU in bytes.
	void (*fpu_save)(void* dst);	   // Function to call when saving the FPU state.
	void (*fpu_restore)(void* dst);	   // Function to call when restoring the FPU state.
#elif defined(CONFIG_arch_riscv64)
	u32 hart_id;	// Hart CPU ID.
#endif
} ATTR(aligned(arch_page_size)) Cpu;

#ifdef CONFIG_smp
#define MAX_CPUS 1024
#else
#define MAX_CPUS 1
#endif

extern Cpu per_cpu_data[MAX_CPUS];

// Initializes the platform for use by the kernel and boot routines.
void arch_early_init();

// Initializes the rest of the platform after the boot routines have completed.
void arch_init(BootInfo* info);

// Initializes a single processor.
// `cpu`: Information about the CPU that has to be enabled.
void arch_init_cpu(Cpu* cpu, Cpu* boot_cpu);

// Disables a single processor.
bool arch_stop_cpu(usize id);

// Halts all CPUs.
ATTR(noreturn) void arch_stop();

// Writes all registers to the current output stream.
void arch_dump_registers(Context* regs);

// Gets processor metadata.
Cpu* arch_current_cpu();
