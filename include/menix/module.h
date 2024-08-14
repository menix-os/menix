// Code modularization

#pragma once

#include <menix/common.h>
#include <menix/log.h>

#if defined(MODULE_TYPE) && defined(MODULE_NAME)
#define module_log(fmt, ...) kmesg(MODULE_NAME ": " fmt, ##__VA_ARGS__)
#else
#define module_log(fmt, ...) kmesg(fmt, ##__VA_ARGS__)
#endif

typedef i32 (*ModuleInitFn)(void);
typedef void (*ModuleExitFn)(void);

// Module metadata and init/exit hooks for loading modules.
typedef struct ATTR(packed) ATTR(aligned(0x10))
{
	ModuleInitFn init;				// Called to initialize the module. Should return 0 upon success.
	ModuleExitFn exit;				// Called to unload the module.
	const char name[64];			// Name of the module.
	const char author[64];			// Author of this module.
	const char description[128];	// Information about this module.
	const char license[64];			// License information.
} Module;

// Defines a new module. Modules should use this at the end of their source to export the entry.
#define MODULE ATTR(used) ATTR(section(".mod")) static const Module this_module

// Defines a new module function.
#define MODULE_FN ATTR(used) static

// Add optional module information.
#define MODULE_META(name, value) __PASTE(name) "=" __PASTE(value) "\x1E"

// Add all module information that is provided by the build system.
#define MOULE_META_COMMON .author = MODULE_AUTHOR, .description = MODULE_DESCRIPTION, .license = MODULE_LICENSE

// Initialize all modules and their subsystems.
void module_init();

// Clean up all modules, disconnect and disable them all.
void module_fini();
