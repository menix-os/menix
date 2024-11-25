// Firmware and platform initialization

#include <menix/common.h>
#include <menix/system/fw.h>

#ifdef CONFIG_pci
#include <menix/system/pci/pci.h>
#endif

#ifdef CONFIG_acpi
#include <menix/system/acpi/acpi.h>
#endif

void fw_init(BootInfo* info)
{
#ifdef CONFIG_device_tree
	dt_init(info->fdt_blob);
#endif

#ifdef CONFIG_acpi
	acpi_init(info->acpi_rsdp);
#endif
}
