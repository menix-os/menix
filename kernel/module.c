// Module and sub-system initialization.

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
	module_log("Initializing subsystems...\n");
#ifdef CONFIG_pci
	pci_init();
#endif

	// Calculate the module count.
	const usize module_count = SECTION_SIZE(mod) / sizeof(Module);
	const Module* modules = (Module*)SECTION_START(mod);

	// Initialize all built-in modules.
	for (usize i = 0; i < module_count; i++)
	{
		module_log("Loading \"%s\"\n", modules[i].name);
		const i32 ret = modules[i].init();
		if (ret != 0)
			module_err("\"%s\" failed to initialize with error code %i!\n", modules[i].name, ret);
	}
}

void module_fini()
{
	// Calculate the module count.
	const usize module_count = SECTION_SIZE(mod) / sizeof(Module);
	const Module* modules = (Module*)SECTION_START(mod);

	// Clean up all modules.
	for (usize i = 0; i < module_count; i++)
	{
		module_log("Unloading \"%s\"\n", modules[i].name);
		modules[i].exit();
	}

	// Shutodwn all subsystem managers.
#ifdef CONFIG_pci
	pci_fini();
#endif
}
