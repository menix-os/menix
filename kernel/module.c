//? Driver initialization

#include <menix/common.h>
#include <menix/drv/pci.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/self.h>

// We need to see the location and size of the .mod section.
SECTION_DECLARE_SYMBOLS(mod)

void module_init()
{
	// Initialize all subsystem managers.
#ifdef CONFIG_pci
	pci_init();
#endif

	// Calculate the driver count.
	const uint32_t module_count = SECTION_SIZE(mod) / sizeof(Module);
	const Module*  modules = (Module*)SECTION_START(mod);

	// TODO: Use Device Tree to filter compatible strings.
	// TODO: Bind drivers to devices.

	// Initialize all modules.
	for (size_t i = 0; i < module_count; i++)
	{
		const int32_t ret = modules[i].init();
		if (ret)
			kmesg(LOG_ERR, "Module \"%s\" failed to initialize with error code %i!\n", modules[i].name, ret);
	}
}

void module_fini()
{
	// Calculate the driver count.
	const uint32_t module_count = SECTION_SIZE(mod) / sizeof(Module);
	const Module*  modules = (Module*)SECTION_START(mod);

	// Clean up all modules.
	for (size_t i = 0; i < module_count; i++)
	{
		modules[i].exit();
	}

	// Shutodwn all subsystem managers.
#ifdef CONFIG_pci
	pci_fini();
#endif
}
