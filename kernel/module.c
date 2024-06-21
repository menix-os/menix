//? Driver initialization

#include <menix/common.h>
#include <menix/drv/pci.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/stdio.h>

void module_init()
{
	// Initialize all subsystem managers.
	pci_init();

	// Calculate the driver count.
	const uint32_t module_count = (&__ld_sect_mod_end - &__ld_sect_mod_start) / sizeof(Module);
	const Module*  modules = (Module*)&__ld_sect_mod_start;

	// TODO: Use Device Tree to filter compatible strings.
	// TODO: Bind drivers to devices.

	// Initialize all modules.
	for (size_t i = 0; i < module_count; i++)
	{
		const int32_t ret = modules[i].init();
		if (ret)
			klog(LOG_ERR, "Module \"%s\" failed to initialize with error code %i!\n", modules[i].name, ret);
	}
}

void module_fini()
{
	const uint32_t module_count = (&__ld_sect_mod_end - &__ld_sect_mod_start) / sizeof(Module);
	const Module*  modules = (Module*)&__ld_sect_mod_start;

	// Clean up all modules.
	for (size_t i = 0; i < module_count; i++)
	{
		modules[i].exit();
	}
}
