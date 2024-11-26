// Dynamic System Tree

#include <menix/common.h>
#include <menix/system/dst/core.h>

#ifdef CONFIG_acpi
#include <menix/system/acpi/acpi.h>
#endif

void dst_init(EarlyBootInfo* info)
{
#ifdef CONFIG_device_tree
	dt_init(info->fdt_blob);
#endif

#ifdef CONFIG_acpi
	acpi_init(info->acpi_rsdp);
#endif
}
