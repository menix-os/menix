// Interrupt descriptor table setting

#include <menix/memory/mmio.h>
#include <menix/system/arch.h>
#include <menix/system/interrupts.h>
#include <menix/system/sch/scheduler.h>
#include <menix/util/log.h>

#include <apic.h>
#include <gdt.h>
#include <idt.h>
#include <interrupts.h>
#include <io.h>
#include <pic.h>

[[gnu::aligned(0x1000)]] static IdtDesc idt_table[IDT_SIZE];
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

// Directly called by the IDT handler.
// Referenced by arch.s
Context* idt_dispatcher(usize isr, Context* regs)
{
	CpuInfo* cpu = arch_current_cpu();

	Context* result = regs;
	// If the IDT slot is occupied, call that function directly.
	if (likely(cpu->idt_callbacks[isr]))
		result = cpu->idt_callbacks[isr](isr, regs);
	// If it is not, call the generic handler instead.
	else
		irq_generic_handler(cpu->idt_to_irq_map[isr]);

	return result;
}

// Deliberately does nothing.
static Context* idt_noop(usize isr, Context* regs)
{
	return regs;
}

void idt_reload()
{
	sch_stop_preemption();

	CpuInfo* cpu = arch_current_cpu();

	// Set known ISRs (Exceptions, timer, syscall) on this core.
	for (usize i = 0; i < 32; i++)
		cpu->idt_callbacks[i] = idt_noop;	 // Exceptions
	cpu->idt_callbacks[0x3] = interrupt_debug_handler;
	cpu->idt_callbacks[0x6] = interrupt_ud_handler;
	cpu->idt_callbacks[0xE] = interrupt_pf_handler;
	cpu->idt_callbacks[INT_TIMER] = timer_handler;
	cpu->idt_callbacks[INT_SYSCALL] = syscall_handler;

	idtr.limit = sizeof(idt_table) - 1;	   // Limit is the last entry, not total size.
	idtr.base = idt_table;
	asm volatile("lidt %0" ::"m"(idtr));

	sch_start_preemption();
}

extern void* arch_int_table[IDT_SIZE];

void idt_init()
{
	for (usize i = 0; i < IDT_SIZE; i++)
	{
		idt_set(i, arch_int_table[i], IDT_TYPE(0, IDT_GATE_INT));
	}
}
