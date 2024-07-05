//? Code modularization

#pragma once

#include <menix/common.h>

typedef int32_t (*ModuleInitFn)(void);
typedef void	(*ModuleExitFn)(void);

// Module metadata and init/fini hooks for loading modules.
typedef struct
{
	ModuleInitFn init;		  // Called to initialize the module. Should return 0 upon success.
	ModuleExitFn exit;		  // Called to unload the module.
	const char	 name[64];	  // Name of the module.
	const char	 meta[];	  // Optional information about the module (Can be NULL).
} ATTR(packed) Module;

// Defines a new module. Modules should use this at the end of their source to export the entry.
#define MODULE	  ATTR(used) ATTR(section(".mod")) static const Module this_module
// Defines a new module function.
#define MODULE_FN ATTR(used) static

// Add optional module information.
#define MOULE_META(name, value) __PASTE(name) "=" __PASTE(value) "\n"

// Initialize all drivers and their subsystems.
void module_init();

// Clean up all drivers, disconnect and disable them all.
void module_fini();
