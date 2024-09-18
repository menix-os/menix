// Firmware and platform initialization

#include <menix/common.h>
#include <menix/system/fw.h>

#ifdef CONFIG_acpi
#include <menix/system/acpi/acpi.h>
#endif

static BootInfo* fw_boot_info;

void fw_init(BootInfo* info)
{
	fw_boot_info = info;

#ifdef CONFIG_acpi
	acpi_init(info->acpi_rsdp);
#elif defined(CONFIG_device_tree)
	dt_init(info->fdt_blob);
#else
#warning "No firmware selected! This may be a configuration issue!"
#endif
}

BootInfo* fw_get_boot_info()
{
	return fw_boot_info;
}
