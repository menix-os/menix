// Code modularization

#pragma once

#include <menix/boot.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/thread/elf.h>

#if defined(MODULE_TYPE) && defined(MODULE_NAME)
#define module_log(fmt, ...) kmesg("[" MODULE_NAME "]\t" fmt, ##__VA_ARGS__)
#else
#define module_log(fmt, ...) kmesg("[Module]\t" fmt, ##__VA_ARGS__)
#endif

// Defines a new module. Modules should use this at the end of their source to export the entry.
#define MODULE ATTR(used) ATTR(section(".mod")) static const Module this_module

// Defines a new module function.
#define MODULE_FN ATTR(used) static

// Add all module information that is provided by the build system.
#define MODULE_META .author = MODULE_AUTHOR, .description = MODULE_DESCRIPTION, .license = MODULE_LICENSE

// Adds a list of dependencies to the module info.
#define MODULE_DEPS(list) .dependencies = list, .num_dependencies = ARRAY_SIZE(list)

// Default values for the module struct.
#define MODULE_DEFAULT(init_fn, exit_fn, deps) \
	MODULE = {.name = MODULE_NAME, .init = init_fn, .exit = exit_fn, MODULE_META, MODULE_DEPS(deps)}

typedef i32 (*ModuleInitFn)(void);
typedef void (*ModuleExitFn)(void);

// Module metadata and init/exit hooks for loading modules.
typedef struct ATTR(packed) ATTR(aligned(0x20))
{
	const char name[64];			// Name of the module.
	const char author[64];			// Author(s) of this module.
	const char description[128];	// Information about this module.
	const char license[48];			// License information.
	ModuleInitFn init;				// Called to initialize the module. Should return 0 upon success.
	ModuleExitFn exit;				// Called to unload the module (Optional, as not every module can be unloaded).
	const char** dependencies;		// A list of modules this module depends on.
	usize num_dependencies;			// Amount of dependencies.
} Module;

// Keeps track of memory allocated by a module at runtime.
typedef struct
{
	char file_path[256];	// Full file path to the module, or NULL if built-in.
	Module* module;			// Underlying information.
	usize num_maps;			// Amount of mappings used.
	struct
	{
		void* address;	  // Base address of the mapping.
		usize size;		  // Bytes allocated.
	} maps[16];			  // Mapping list of dynamically allocated pages.
	bool loaded;		  // If the init() function has been called.
} LoadedModule;

// Initialize all modules and their subsystems.
void module_init(BootInfo* info);

// Clean up all modules, disconnect and disable them all.
void module_fini();

// Registers a module to the list of loaded modules.
void module_register(LoadedModule* module);

// Loads a previously registered module.
i32 module_load(const char* name);

// Unloads a relocatable module.
// `name`: The name of the module.
void module_unregister(const char* name);

// Gets the information of a module.
LoadedModule* module_get(const char* name);

// Registers a symbol.
void module_register_symbol(const char* name, Elf_Sym* symbol);

// Gets a registered symbol.
Elf_Sym* module_get_symbol(const char* name);
