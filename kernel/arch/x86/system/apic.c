// x86 advanced programmable interrupt controller

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/system/arch.h>
#include <menix/thread/scheduler.h>

#include <apic.h>
#include <io.h>
#include <pic.h>

static PhysAddr lapic_addr = 0;
static bool has_x2apic = 0;

void apic_init()
{
	// Remap IRQs so they start at 0x20 since interrupts 0x00..0x1F are used by CPU exceptions.
	arch_x86_write8(PIC1_COMMAND_PORT, 0x11);	 // ICW1: Begin initialization and set cascade mode.
	arch_x86_write8(PIC1_DATA_PORT, 0x20);		 // ICW2: Set where interrupts should be mapped to (0x20-0x27).
	arch_x86_write8(PIC1_DATA_PORT, 0x04);		 // ICW3: Connect IRQ2 (0x04) to the slave PIC.
	arch_x86_write8(PIC1_DATA_PORT, 0x01);		 // ICW4: Set the PIC to operate in 8086/88 mode.
	arch_x86_write8(PIC1_DATA_PORT, 0xFF);		 // Mask all interrupts.

	// Same for the slave PIC.
	arch_x86_write8(PIC2_COMMAND_PORT, 0x11);	 // ICW1: Begin initialization.
	arch_x86_write8(PIC2_DATA_PORT, 0x28);		 // ICW2: Set where interrupts should be mapped to (0x28-0x2F).
	arch_x86_write8(PIC2_DATA_PORT, 0x02);		 // ICW3: Connect to master PIC at IRQ2.
	arch_x86_write8(PIC2_DATA_PORT, 0x01);		 // ICW4: Set the PIC to operate in 8086/88 mode.
	arch_x86_write8(PIC2_DATA_PORT, 0xFF);		 // Mask all interrupts.
}

void apic_send_eoi()
{
	// TODO
}

static u32 ioapic_read(PhysAddr ioapic_address, usize reg)
{
	write16(pm_get_phys_base() + ioapic_address, reg & 0xFF);
	return read16(pm_get_phys_base() + ioapic_address + 16);
}

static void ioapic_write(PhysAddr ioapic_address, usize reg, u32 data)
{
	write16(pm_get_phys_base() + ioapic_address, reg & 0xFF);
	write16(pm_get_phys_base() + ioapic_address + 16, data);
}

void timer_handler(CpuRegisters* regs)
{
	asm_interrupt_disable();

	// TODO
	scheduler_reschedule(regs);

	asm_interrupt_enable();
}
