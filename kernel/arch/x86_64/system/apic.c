// x86 advanced programmable interrupt controller

#include <menix/common.h>
#include <menix/io/mmio.h>
#include <menix/memory/pm.h>
#include <menix/system/acpi/madt.h>
#include <menix/system/arch.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/time/clock.h>

#include <apic.h>
#include <io.h>
#include <pic.h>

static PhysAddr lapic_addr = 0;
static bool has_x2apic = 0;
u32 apic_ticks_in_10ms = 0;

void pic_disable()
{
	// Note: We initialize the PIC properly, but completely disable it and use the APIC in favor of it.
	// Remap IRQs so they start at 0x20 since interrupts 0x00..0x1F are used by CPU exceptions.
	asm_write8(PIC1_COMMAND_PORT, 0x11);	// ICW1: Begin initialization and set cascade mode.
	asm_write8(PIC1_DATA_PORT, 0x20);		// ICW2: Set where interrupts should be mapped to (0x20-0x27).
	asm_write8(PIC1_DATA_PORT, 0x04);		// ICW3: Connect IRQ2 (0x04) to the slave PIC.
	asm_write8(PIC1_DATA_PORT, 0x01);		// ICW4: Set the PIC to operate in 8086/88 mode.
	asm_write8(PIC1_DATA_PORT, 0xFF);		// Mask all interrupts.

	// Same for the slave PIC.
	asm_write8(PIC2_COMMAND_PORT, 0x11);	// ICW1: Begin initialization.
	asm_write8(PIC2_DATA_PORT, 0x28);		// ICW2: Set where interrupts should be mapped to (0x28-0x2F).
	asm_write8(PIC2_DATA_PORT, 0x02);		// ICW3: Connect to master PIC at IRQ2.
	asm_write8(PIC2_DATA_PORT, 0x01);		// ICW4: Set the PIC to operate in 8086/88 mode.
	asm_write8(PIC2_DATA_PORT, 0xFF);		// Mask all interrupts.
}
static inline u32 reg_to_x2apic(u32 reg)
{
	return ((reg == 0x310) ? 0x30 : (reg >> 4)) + 0x800;
}

u32 lapic_read(u32 reg)
{
	if (has_x2apic)
		return asm_rdmsr(reg_to_x2apic(reg));

	return mmio_read16(pm_get_phys_base() + lapic_addr + reg);
}

void lapic_write(u32 reg, u32 value)
{
	if (has_x2apic)
		asm_wrmsr(reg_to_x2apic(reg), value);
	else
		mmio_write16(pm_get_phys_base() + lapic_addr + reg, value);
}

void lapic_init(usize cpu_id)
{
	u64 apic_msr = asm_rdmsr(0x1B);
	// Set APIC enable flag
	apic_msr |= 1 << 11;
	u32 a = 0, b = 0, c = 0, d = 0;
	asm_cpuid(1, 0, a, b, c, d);
	if (c & CPUID_1C_X2APIC)
	{
		// Set X2APIC flag
		has_x2apic = true;
		apic_msr |= 1 << 10;
	}
	else
	{
		// TODO
		return;
	}

	asm_wrmsr(0x1B, apic_msr);

	// Initialize local APIC
	lapic_write(0x80, 0);
	lapic_write(0xF0, lapic_read(0xF0) | 0x100);
	if (!has_x2apic)
	{
		lapic_write(0xE0, 0xF0000000);
		lapic_write(0xD0, lapic_read(0x20));
	}

	// Set up APIC timer

	// Tell APIC timer to divide by 16
	lapic_write(0x3E0, 3);
	// Set timer init counter to -1
	lapic_write(0x380, 0xFFFFFFFF);

	// See how many ticks pass in 10 milliseconds.
	clock_wait(10 * 1000000);

	// Stop the APIC timer
	lapic_write(0x320, 0x10000);

	// How much the APIC timer ticked in 10ms
	apic_ticks_in_10ms = 0xFFFFFFFF - lapic_read(0x390);

	// Make sure interrupts are off.
	asm_interrupt_disable();

	// Start timer as periodic on IRQ 0
	lapic_write(0x320, INT_TIMER | 0x20000);
	// With divider 16
	lapic_write(0x3E0, 3);
	lapic_write(0x380, apic_ticks_in_10ms);
}

void apic_send_eoi()
{
	lapic_write(0xB0, 0);
}

Context* timer_handler(usize isr, Context* regs, void* data)
{
	Context* new_context = sch_reschedule(regs);
	apic_send_eoi();
	return new_context;
}
