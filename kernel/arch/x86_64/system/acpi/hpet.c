// High Precision Event Timer table

#include <menix/memory/pm.h>
#include <menix/system/acpi/acpi.h>
#include <menix/system/acpi/types.h>
#include <menix/system/time/clock.h>
#include <menix/util/log.h>

#include <hpet.h>
#include <uacpi/acpi.h>
#include <uacpi/tables.h>

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

	print_log("acpi: Enabled HPET.\n");

	clock_register(&hpet.cs);
}

void hpet_init()
{
	uacpi_table hpet_table;
	kassert(!uacpi_table_find_by_signature("HPET", &hpet_table), "Failed to get the HPET table!");
	struct acpi_hpet* hpet = hpet_table.ptr;

	print_log("acpi: HPET at 0x%p\n", hpet);

	hpet_setup(hpet->address.address);
	uacpi_table_unref(&hpet_table);
}
