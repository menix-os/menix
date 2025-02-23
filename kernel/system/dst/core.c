// Dynamic System Tree

#include <menix/common.h>
#include <menix/system/acpi/acpi.h>
#include <menix/system/dst/core.h>
#include <menix/util/cmd.h>
#include <menix/util/log.h>

void dst_init(BootInfo* info)
{
	if (cmd_get_usize("acpi", true))
	{
		if (info->acpi_rsdp)
			acpi_init(info->acpi_rsdp);
		else
			print_warn("dst: Unable to configure using ACPI, no RSDP given!\n");
	}
	else
		print_warn("dst: ACPI is disabled!\n");
}
