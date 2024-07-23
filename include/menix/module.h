// Code modularization

#pragma once

#include <menix/common.h>
#include <menix/log.h>

#if defined(MODULE_TYPE) && defined(MODULE_NAME)
#define module_log(fmt, ...) kmesg("[" MODULE_NAME "] " fmt, ##__VA_ARGS__)
#define module_err(fmt, ...) kmesg("[" MODULE_NAME "] " fmt, ##__VA_ARGS__)
#define module_dbg(fmt, ...) kmesg("[" MODULE_NAME "] " fmt, ##__VA_ARGS__)
#else
#define module_log(fmt, ...) kmesg("[Module] " fmt, ##__VA_ARGS__)
#define module_err(fmt, ...) kmesg("[Module] Error: ", fmt, ##__VA_ARGS__)
#define module_dbg(fmt, ...) kmesg("[Module] " fmt, ##__VA_ARGS__)
#endif

typedef int32_t (*ModuleInitFn)(void);
typedef void (*ModuleExitFn)(void);

// Module metadata and init/exit hooks for loading modules.
typedef struct ATTR(packed)
{
	ModuleInitFn init;		// Called to initialize the module. Should return 0 upon success.
	ModuleExitFn exit;		// Called to unload the module.
	const char name[64];	// Name of the module.
	const char meta[];		// Optional information about the module (Can be NULL).
} Module;

// Defines a new module. Modules should use this at the end of their source to export the entry.
#define MODULE	  ATTR(used) ATTR(section(".mod")) static const Module this_module
// Defines a new module function.
#define MODULE_FN ATTR(used) static

// Add optional module information.
#define MODULE_META(name, value) __PASTE(name) "=" __PASTE(value) "\n"

// Add all module information that is provided by the build system.
#define MOULE_META_COMMON \
	MODULE_META("author", MODULE_AUTHOR) \
	MODULE_META("description", MODULE_DESCRIPTION) MODULE_META("license", MODULE_LICENSE)

// Initialize all drivers and their subsystems.
void module_init();

// Clean up all drivers, disconnect and disable them all.
void module_fini();
