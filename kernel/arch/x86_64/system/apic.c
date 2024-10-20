// x86 advanced programmable interrupt controller

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/system/acpi/madt.h>
#include <menix/system/arch.h>
#include <menix/thread/scheduler.h>

#include <apic.h>
#include <io.h>
#include <pic.h>

static PhysAddr lapic_addr = 0;
static bool has_x2apic = 0;
u32 tick_in_10ms = 0;

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

	// apic_redirect_irq(0, 48);
}

static u32 ioapic_read(PhysAddr ioapic_address, usize reg)
{
	mmio_write16(pm_get_phys_base() + ioapic_address, reg & 0xFF);
	return mmio_read16(pm_get_phys_base() + ioapic_address + 16);
}

static void ioapic_write(PhysAddr ioapic_address, usize reg, u32 data)
{
	mmio_write16(pm_get_phys_base() + ioapic_address, reg & 0xFF);
	mmio_write16(pm_get_phys_base() + ioapic_address + 16, data);
}

static MadtIoApic* get_ioapic_by_gsi(i32 gsi)
{
	for (usize i = 0; i < madt_ioapic_list.length; i++)
	{
		MadtIoApic* ioapic = madt_ioapic_list.items[i];
		if (ioapic->gsi_base <= gsi &&
			ioapic->gsi_base + ((ioapic_read(ioapic->ioapic_addr, 1) & 0xFF0000) >> 16) > gsi)
			return ioapic;
	}

	return NULL;
}

static void ioapic_redirect_gsi(u32 gsi, u8 vec, u16 flags)
{
	// Get I/O APIC address of the GSI
	usize io_apic = get_ioapic_by_gsi(gsi)->ioapic_addr;

	u32 low_index = 0x10 + (gsi - get_ioapic_by_gsi(gsi)->gsi_base) * 2;
	u32 high_index = low_index + 1;

	u32 high = ioapic_read(io_apic, high_index);

	// Set APIC ID
	high &= ~0xFF000000;
	high |= ioapic_read(io_apic, 0) << 24;
	ioapic_write(io_apic, high_index, high);

	u32 low = ioapic_read(io_apic, low_index);

	low &= ~(1 << 16);
	low &= ~(1 << 11);
	low &= ~0x700;
	low &= ~0xFF;
	low |= vec;

	if (flags & 2)
		low |= 1 << 13;

	if (flags & 8)
		low |= 1 << 15;

	ioapic_write(io_apic, low_index, low);
}

static inline u32 reg_to_x2apic(u32 reg)
{
	return ((reg == 0x310) ? 0x30 : (reg >> 4)) + 0x800;
}

void apic_redirect_irq(u32 irq, u8 interrupt)
{
	for (u32 i = 0; i < madt_iso_list.length; i++)
	{
		if (madt_iso_list.items[i]->irq_source == irq)
		{
			ioapic_redirect_gsi(madt_iso_list.items[i]->gsi, interrupt, madt_iso_list.items[i]->flags);
			return;
		}
	}
	ioapic_redirect_gsi(irq, interrupt, 0);
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

static void lapic_set_nmi(u8 vec, u8 current_processor_id, u8 processor_id, u16 flags, u8 lint)
{
	// A value of 0xFF means all the processors
	if (processor_id != 0xFF)
	{
		if (current_processor_id != processor_id)
			return;
	}

	// Set to raise in vector number "vec" and set NMI flag
	u32 nmi = 0x400 | vec;

	// Set to active low if needed
	if (flags & 2)
		nmi |= 1 << 13;

	// Set to level triggered if needed
	if (flags & 8)
		nmi |= 1 << 15;

	// Use the proper LINT register
	if (lint == 0)
		lapic_write(0x350, nmi);
	else if (lint == 1)
		lapic_write(0x360, nmi);
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
		has_x2apic = true;
		// Set X2APIC flag
		apic_msr |= 1 << 10;
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

	// Set NMIs according to the MADT
	for (int i = 0; i < madt_nmi_list.length; i++)
	{
		MadtNmi* nmi = madt_nmi_list.items[i];
		lapic_set_nmi(2, cpu_id, nmi->acpi_id, nmi->flags, nmi->lint);
	}

	// Set up APIC timer

	// Tell APIC timer to divide by 16
	lapic_write(0x3E0, 3);
	// Set timer init counter to -1
	lapic_write(0x380, 0xFFFFFFFF);

	// timer_sleep(10);

	// Stop the APIC timer
	lapic_write(0x320, 0x10000);

	// How much the APIC timer ticked in 10ms
	tick_in_10ms = 0xFFFFFFFF - lapic_read(0x390);

	// Start timer as periodic on IRQ 0
	lapic_write(0x320, 32 | 0x20000);
	// With divider 16
	lapic_write(0x3E0, 3);
	lapic_write(0x380, tick_in_10ms / 10);
}

void apic_send_eoi()
{
	lapic_write(0xB0, 0);
}

void timer_handler(Context* regs)
{
	asm_interrupt_disable();

	// TODO
	scheduler_reschedule(regs);

	apic_send_eoi();
	asm_interrupt_enable();
}
