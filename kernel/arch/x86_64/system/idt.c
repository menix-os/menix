// Interrupt descriptor table setting

#include <menix/memory/mmio.h>
#include <menix/system/arch.h>
#include <menix/system/interrupts.h>
#include <menix/util/log.h>

#include <apic.h>
#include <gdt.h>
#include <idt.h>
#include <interrupts.h>
#include <io.h>
#include <pic.h>

[[gnu::aligned(0x1000)]] static IdtDesc idt_table[IDT_MAX_SIZE];
[[gnu::aligned(0x10)]] static IdtRegister idtr;

void idt_set(u8 idx, void* handler, u8 type_attr)
{
	IdtDesc* const target = idt_table + idx;
	const usize ptr = (usize)handler;

	target->base_0_15 = ptr & 0xFFFF;
	target->base_16_31 = (ptr >> 16) & 0xFFFF;
	target->selector = offsetof(Gdt, kernel_code);
	target->type = type_attr;
	target->reserved = 0;
#if ARCH_BITS >= 64
	target->base_32_63 = (ptr >> 32) & 0xFFFFFFFF;
	target->reserved2 = 0;
#endif
}

void idt_reload()
{
	const usize cpu = arch_current_cpu()->id;

	// Set IRQ handlers for known ISRs (Exceptions, timer, syscall) on this core.
	isr_register_handler(cpu, 0x3, interrupt_debug_handler, NULL);
	isr_register_handler(cpu, 0x6, interrupt_ud_handler, NULL);
	isr_register_handler(cpu, 0xE, interrupt_pf_handler, NULL);
	isr_register_handler(cpu, INT_TIMER, timer_handler, NULL);
	isr_register_handler(cpu, INT_SYSCALL, syscall_handler, NULL);

	idtr.limit = sizeof(idt_table) - 1;	   // Limit is the last entry, not total size.
	idtr.base = idt_table;
	asm volatile("lidt %0" ::"m"(idtr));
}

extern void* arch_int_table[IDT_MAX_SIZE];

void idt_init()
{
	for (usize i = 0; i < IDT_MAX_SIZE; i++)
	{
		idt_set(i, arch_int_table[i], IDT_TYPE(0, IDT_GATE_INT));
	}
}
