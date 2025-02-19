// High Precision Event Timer table

#include <menix/memory/pm.h>
#include <menix/system/acpi/acpi.h>
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
static void hpet_reset();

static Hpet hpet = {
	.cs = {.name = "hpet", .get_elapsed_ns = hpet_get_elapsed_ns, .reset = hpet_reset},
};

static usize hpet_get_elapsed_ns()
{
	// Convert femtoseconds to nanoseconds.
	return hpet.regs->main_counter * (hpet.period / 1000000);
}

static void hpet_reset()
{
	hpet.regs->main_counter = 0;
}

static void hpet_setup(PhysAddr addr)
{
	print_log("acpi: HPET at 0x%p\n", addr);
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
	if (uacpi_table_find_by_signature("HPET", &hpet_table) != UACPI_STATUS_OK)
	{
		print_error("acpi: Failed to get the HPET table!\n");
		return;
	}

	struct acpi_hpet* hpet = hpet_table.ptr;

	hpet_setup(hpet->address.address);
	uacpi_table_unref(&hpet_table);
}
