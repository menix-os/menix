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
	ModuleInitFn   init;		   // Called to initialize the module. Should return 0 upon success.
	ModuleExitFn   exit;		   // Called to unload the module.
	void* nullable data;		   // Driver data. Can be null if not required.
	const char*	   name;		   // Name of the module.
	const char*	   author;		   // Author(s) of the module.
	const char*	   description;	   // Description of the module.
} ATTR(packed) Module;

// Define a new module. Modules should use this at the end of their source.
#define MENIX_MODULE ATTR(used) ATTR(section(".mod")) static const Module __reserved__module

// Sets the common meta data in the Module struct.
#define MENIX_MODULE_META .name = MODULE_NAME, .author = MODULE_AUTHOR, .description = MODULE_DESCRIPTION

// Start and end markers of the module section. Defined in the linker script.
// To use them, dereference.
extern const uint8_t __ld_sect_mod_start;
extern const uint8_t __ld_sect_mod_end;

// Initialize all drivers.
void module_init();

// Clean up all drivers, disconnect and disable them all.
void module_fini();
