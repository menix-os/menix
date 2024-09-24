// High Precision Event Timer table

#include <menix/system/acpi/acpi.h>
#include <menix/system/acpi/types.h>
#include <menix/util/log.h>

#include <hpet.h>

static AcpiHpet* hpet_table;

void hpet_init()
{
	kassert(hpet_table = acpi_find_table("HPET", 0), "ACPI tables didn't contain an HPET table!");
}
