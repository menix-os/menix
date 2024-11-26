// High Precision Event Timer table

#include <menix/memory/pm.h>
#include <menix/system/acpi/acpi.h>
#include <menix/system/acpi/types.h>
#include <menix/system/time/clock.h>
#include <menix/util/log.h>

#include <hpet.h>

typedef struct
{
	volatile HpetRegisters* regs;
	u32 period;
	ClockSource cs;
} Hpet;

static usize hpet_get_elapsed_ns();

static Hpet hpet = {
	.cs = {"hpet", hpet_get_elapsed_ns},
};

static usize hpet_get_elapsed_ns()
{
	// Convert femtoseconds to nanoseconds.
	return hpet.regs->main_counter * (hpet.period / 1000000);
}

static void hpet_setup(PhysAddr addr)
{
	hpet.regs = pm_get_phys_base() + addr;

	// Get the period.
	hpet.period = (u32)(hpet.regs->capabilities >> 32);

	// Enable timer.
	hpet.regs->configuration |= 1;

	clock_register(&hpet.cs);
}

void hpet_init()
{
	AcpiHpet* hpet_table = acpi_find_table("HPET", 0);
	kassert(hpet_table, "ACPI tables didn't contain an HPET table!");

	acpi_log("HPET at 0x%p\n", hpet_table->address);

	hpet_setup(hpet_table->address.address);
}
