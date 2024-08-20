// Module and sub-system initialization.

#include <menix/common.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/util/self.h>

// We need to see the location and size of the .mod section.
SECTION_DECLARE_SYMBOLS(mod)

void module_init()
{
	// Check if the .mod section size is sane.
	kassert(SECTION_SIZE(mod) % alignof(Module) == 0, ".mod section has a bogus size! This might be a linker issue.\n");

	// Calculate the module count.
	const usize module_count = SECTION_SIZE(mod) / sizeof(Module);
	const Module* modules = (Module*)SECTION_START(mod);

	// Initialize all built-in modules.
	kmesg("Loading %i built-in modules.\n", module_count);
	for (usize i = 0; i < module_count; i++)
	{
		kmesg("Loading \"%s\": %s (Author: %s, License: %s)\n", modules[i].name, modules[i].description,
			  modules[i].author, modules[i].license);
		const i32 ret = modules[i].init();
		if (ret != 0)
			kmesg("\"%s\" failed to initialize with error code %i!\n", modules[i].name, ret);
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
		if (modules[i].exit)
		{
			kmesg("Unloading \"%s\"\n", modules[i].name);
			modules[i].exit();
		}
	}
}
