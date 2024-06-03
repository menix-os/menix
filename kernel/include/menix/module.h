/*---------------------------------
Utilities for kernel modularization
---------------------------------*/

#pragma once

#include <menix/common.h>
#include <menix/stdint.h>

// For attributes that are only active when a module is actually modularized and not built-in.
#ifdef MODULE
#define MENIX_MODULE_ATTR(x) MENIX_ATTR(x)
#else
#define MENIX_MODULE_ATTR(x)
#endif

// Module information. Every module needs this exactly once!
#define MENIX_MODULE_INFO static const MENIX_MODULE_ATTR(used) MENIX_MODULE_ATTR(section(".mod")) Module module_info =

typedef int32_t (*ModLoadFn)();
typedef void	(*ModExitFn)();

// Module information.
typedef struct MENIX_ATTR(packed)
{
	// Function to call during module loading. Should return a status code.
	const ModLoadFn load;
	// Function to call during module unloading.
	const ModExitFn exit;
} Module;
