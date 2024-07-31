// x86 platform initialization

#include <menix/arch.h>
#include <menix/log.h>
#include <menix/memory/vm.h>
#include <menix/serial.h>

#include <gdt.h>
#include <idt.h>
#include <interrupts.h>

void arch_early_init()
{
	gdt_init();
	idt_init();
	// Init COM1 for serial output.
	serial_initialize();
}

void arch_init(BootInfo* info)
{
	vm_init(&info->memory_map);
}

void arch_stop(BootInfo* info)
{
	asm_interrupt_disable();
	asm volatile("hlt");
}

#define _GET_REGISTER(val, reg) asm volatile("mov %%" #reg ", %0" : "=m"(val.reg))

void arch_dump_registers()
{
	CpuRegisters regs;
	_GET_REGISTER(regs, rax);
	_GET_REGISTER(regs, rbx);
	_GET_REGISTER(regs, rcx);
	_GET_REGISTER(regs, rdx);

	_GET_REGISTER(regs, rsi);
	_GET_REGISTER(regs, rdi);
	_GET_REGISTER(regs, rbp);
	_GET_REGISTER(regs, rsp);

	_GET_REGISTER(regs, r8);
	_GET_REGISTER(regs, r9);
	_GET_REGISTER(regs, r10);
	_GET_REGISTER(regs, r11);

	_GET_REGISTER(regs, r12);
	_GET_REGISTER(regs, r13);
	_GET_REGISTER(regs, r14);
	_GET_REGISTER(regs, r15);

	kmesg("rax: 0x%p rbx: 0x%p rcx: 0x%p rdx: 0x%p\n", regs.rax, regs.rbx, regs.rcx, regs.rdx);
	kmesg("rsi: 0x%p rdi: 0x%p rbp: 0x%p rsp: 0x%p\n", regs.rsi, regs.rdi, regs.rbp, regs.rsp);
	kmesg("r8:  0x%p r9:  0x%p r10: 0x%p r11: 0x%p\n", regs.r8, regs.r9, regs.r10, regs.r11);
	kmesg("r12: 0x%p r13: 0x%p r14: 0x%p r15: 0x%p\n", regs.r12, regs.r13, regs.r14, regs.r15);
}
