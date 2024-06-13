/*-------------------
Driver initialization
-------------------*/

#include <menix/common.h>
#include <menix/module.h>
#include <menix/stdio.h>

void module_init()
{
	// Calculate the driver count.
	const uint32_t module_count = (&__ld_sect_mod_end - &__ld_sect_mod_start) / sizeof(Module);
	const Module*  modules = (Module*)&__ld_sect_mod_start;

	// TODO: Use Device Tree to filter compatible strings.
	// TODO: Bind drivers to devices.

	printf("initializing modules:\n---\n");

	// Initialize all modules.
	for (size_t i = 0; i < module_count; i++)
	{
		// Init.
		modules[i].init();
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
