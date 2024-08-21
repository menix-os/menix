// x86 platform initialization

#include <menix/arch.h>
#include <menix/io/serial.h>
#include <menix/log.h>
#include <menix/memory/alloc.h>

#include <gdt.h>
#include <idt.h>
#include <interrupts.h>

static Cpu* cpus;

void arch_early_init()
{
	gdt_init();
	idt_init();
	// Init COM1 for serial output.
	serial_initialize();
}

void arch_init(BootInfo* info)
{
	// Initialize all other processors.
#ifdef CONFIG_smp
	// TODO: Find out processor count.
#else
	cpus = kcalloc(sizeof(Cpu));
#endif
}

void arch_shutdown(BootInfo* info)
{
	// TODO
	arch_stop(info);
}

void arch_stop(BootInfo* info)
{
	asm_interrupt_disable();
	asm volatile("hlt");
}

Cpu* arch_current_cpu()
{
#ifdef CONFIG_smp
	u64 id;
	// The CPU ID is stored in GS (thread local memory).
	asm volatile("mov %%gs:0, %0" : "=r"(id) : : "memory");
	return &cpus[id];
#else
	return &cpus[0];
#endif
}

void arch_dump_registers()
{
	CpuRegisters regs;
	asm_get_register(regs.rax, rax);
	asm_get_register(regs.rbx, rbx);
	asm_get_register(regs.rcx, rcx);
	asm_get_register(regs.rdx, rdx);
	asm_get_register(regs.rsi, rsi);
	asm_get_register(regs.rdi, rdi);
	asm_get_register(regs.rbp, rbp);
	asm_get_register(regs.rsp, rsp);
	asm_get_register(regs.r8, r8);
	asm_get_register(regs.r9, r9);
	asm_get_register(regs.r10, r10);
	asm_get_register(regs.r11, r11);
	asm_get_register(regs.r12, r12);
	asm_get_register(regs.r13, r13);
	asm_get_register(regs.r14, r14);
	asm_get_register(regs.r15, r15);
	asm_get_register(regs.cs, cs);
	asm_get_register(regs.ss, ss);

	kmesg("rax: 0x%p rbx: 0x%p rcx: 0x%p rdx: 0x%p\n", regs.rax, regs.rbx, regs.rcx, regs.rdx);
	kmesg("rsi: 0x%p rdi: 0x%p rbp: 0x%p rsp: 0x%p\n", regs.rsi, regs.rdi, regs.rbp, regs.rsp);
	kmesg("r8:  0x%p r9:  0x%p r10: 0x%p r11: 0x%p\n", regs.r8, regs.r9, regs.r10, regs.r11);
	kmesg("r12: 0x%p r13: 0x%p r14: 0x%p r15: 0x%p\n", regs.r12, regs.r13, regs.r14, regs.r15);
	kmesg("cs:  0x%p ss:  0x%p\n", regs.cs, regs.ss);
}
