/*-----------------
Code modularization
-----------------*/

#pragma once

#include <menix/common.h>
#include <menix/stdint.h>

typedef int32_t (*ModuleInitFn)(void);
typedef void	(*ModuleExitFn)(void);

// Module metadata and init/fini hooks for loading modules.
typedef struct
{
	ModuleInitFn init;			 // Called to initialize the module. Should return 0 upon success.
	ModuleExitFn exit;			 // Called to unload the module.
	const char*	 name;			 // Name of the module.
	const char*	 author;		 // Author(s) of the module.
	const char*	 description;	 // Description of the module.
} ATTR(packed) Module;

// Define a new module. Modules should use this at the end of their source.
#define MENIX_MODULE(...) \
	ATTR(used) ATTR(section(".mod")) static const Module this_module = { \
		__VA_ARGS__, \
		.name = MODULE_NAME, \
		.author = MODULE_AUTHOR, \
		.description = MODULE_DESCRIPTION, \
	}; \
	static_assert(this_module.init != NULL, "Init function has to be set!"); \
	static_assert(this_module.exit != NULL, "Exit function has to be set!")

// Start and end markers of the module section. Defined in the linker script.
// To use them, dereference.
extern const uint8_t __ld_sect_mod_start;
extern const uint8_t __ld_sect_mod_end;

// Initialize all drivers.
void module_init();

// Clean up all drivers, disconnect and disable them all.
void module_fini();
