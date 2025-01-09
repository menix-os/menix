// Dynamic System Tree

#include <menix/common.h>
#include <menix/system/acpi/acpi.h>
#include <menix/system/dst/core.h>
#include <menix/util/log.h>

void dst_init(BootInfo* info)
{
	// TODO
	// dt_init(info->fdt_blob);

	if (info->acpi_rsdp)
		acpi_init(info->acpi_rsdp);
	else
		print_warn("dst: Unable to configure using ACPI, no RSDP given!\n");
}
