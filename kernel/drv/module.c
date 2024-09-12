// Module and sub-system initialization.

#include <menix/common.h>
#include <menix/drv/pci/pci.h>
#include <menix/fs/vfs.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/util/hash_map.h>
#include <menix/util/self.h>

#include <errno.h>
#include <string.h>

#include "menix/abi.h"

#ifdef CONFIG_acpi
#include <menix/drv/acpi/acpi.h>
#endif

// We need to see the location and size of the .mod section.
SECTION_DECLARE_SYMBOLS(mod)

// Stores all loaded modules.
static HashMap(LoadedModule*) module_map;

// Stores all known symbols.
static HashMap(Elf_Sym*) module_symbol_map;

void module_init(BootInfo* info)
{
	// Initialize subsystems.
#ifdef CONFIG_acpi
	acpi_init(info->acpi_rsdp);
#endif
#ifdef CONFIG_pci
	pci_init();
#endif

	// Initialize the module map.
	hashmap_init(module_map, 128);

	// Calculate the module count.
	const usize module_count = SECTION_SIZE(mod) / sizeof(Module);
	Module* const modules = (Module*)SECTION_START(mod);

	// Check if the .mod section size is sane.
	if (SECTION_SIZE(mod) % sizeof(Module) != 0)
		module_log("Ignoring built-in modules: The .mod section has a bogus size of 0x%zx!\n", SECTION_SIZE(mod));
	else
	{
		// Register all built-in modules.
		for (usize i = 0; i < module_count; i++)
		{
			LoadedModule* module_info = kzalloc(sizeof(LoadedModule));
			module_info->module = modules + i;
			module_register(module_info);
		}
	}

	// Add all kernel symbols to the symbol map.
	void* const kernel = elf_get_kernel();
	hashmap_init(module_symbol_map, 128);

	// Get symbol table.
	Elf_Shdr* symtab = elf_get_section(kernel, ".symtab");
	Elf_Sym* symtab_data = kernel + symtab->sh_offset;
	kassert(symtab != NULL, "Couldn't find kernel symbol table!");

	// Get string table.
	Elf_Shdr* strtab = elf_get_section(kernel, ".strtab");
	const char* strtab_data = kernel + strtab->sh_offset;
	kassert(symtab != NULL, "Couldn't find kernel string table!");

	for (usize sym = 0; sym < symtab->sh_size / symtab->sh_entsize; sym++)
	{
		const char* symbol_name = strtab_data + symtab_data[sym].st_name;
		// Only match global symbols.
		if (symtab_data[sym].st_info & (STB_GLOBAL << 4) && symtab_data[sym].st_size != 0)
			module_register_symbol(symbol_name, symtab_data + sym);
	}

	// TODO: Load modules from VFS.
	// Check if /boot/modules exists.
	const char* dyn_modules_path = "/boot/modules/";	// TODO: cmdline override of this value.
	VfsNode* module_path = vfs_get_node(vfs_get_root(), dyn_modules_path, true);
	if (module_path == NULL)
	{
		module_log("Can't find dynamic modules, directory \"%s\" is missing!\n", dyn_modules_path);
		goto skip_dynamic;
	}
	if (!S_ISDIR(module_path->handle->stat.st_mode))
	{
		module_log("Can't find dynamic modules, \"%s\" is not a directory!\n", dyn_modules_path);
		goto skip_dynamic;
	}
	for (usize b = 0; b < module_path->children.capacity; b++)
	{
		if (module_path->children.buckets == NULL)
			break;

		auto bucket = module_path->children.buckets + b;
		for (usize i = 0; i < bucket->count; i++)
		{
			VfsNode* node = bucket->items[i].item;
			// Only get real files.
			if (node->handle == NULL)
				continue;
			if (!S_ISREG(node->handle->stat.st_mode))
				continue;
			char* full_path = kmalloc(512);
			vfs_get_path(node, full_path, 512);
			elf_module_load(full_path);
		}
	}
skip_dynamic:

	// Load every registered module.
	for (usize b = 0; b < module_map.capacity; b++)
	{
		if (module_map.buckets == NULL)
			break;

		auto bucket = module_map.buckets + b;
		for (usize i = 0; i < bucket->count; i++)
		{
			LoadedModule* loaded = bucket->items[i].item;
			i32 ret = module_load(loaded->module->name);
			if (ret != 0)
				module_log("\"%s\" failed to initialize with error code %i!\n", loaded->module->name, ret);
		}
	}
}

void module_fini()
{
	// Check if the .mod section size is sane.
	if (SECTION_SIZE(mod) % sizeof(Module) != 0)
		return;

	// Calculate the module count.
	const usize module_count = SECTION_SIZE(mod) / sizeof(Module);
	const Module* modules = (Module*)SECTION_START(mod);

	// Clean up all modules.
	for (usize i = 0; i < module_count; i++)
	{
		if (modules[i].exit != NULL)
		{
			module_log("Unloading \"%s\"\n", modules[i].name);
			modules[i].exit();
		}
	}

#ifdef CONFIG_pci
	pci_fini();
#endif
}

LoadedModule* module_get(const char* name)
{
	LoadedModule* get = NULL;
	hashmap_get(&module_map, get, name, strlen(name));
	return get;
}

void module_register(LoadedModule* module)
{
	// If the module is already loaded, skip.
	if (module_get(module->module->name))
		return;

	hashmap_insert(&module_map, module->module->name, strlen(module->module->name), module);
	module_log("Registered new module \"%s\"\n", module->module->name);
}

// Loads a previously registered module.
i32 module_load(const char* name)
{
	LoadedModule* loaded;

	// Check if module is registered.
	if (!hashmap_get(&module_map, loaded, name, strlen(name)))
	{
		module_log("Unable to load \"%s\": Not previously registered!\n", name);
		return -ENOENT;
	}
	if (loaded->loaded)
		return 0;

	Module* const mod = loaded->module;
	module_log("Loading module \"%s\" at 0x%p: %s (%s, %s)\n", mod->name, loaded->module, mod->description, mod->author,
			   mod->license);

	// Load all dependencies.
	for (usize i = 0; i < mod->num_dependencies; i++)
	{
		if (module_load(mod->dependencies[i]) != 0)
		{
			module_log("Failed to load \"%s\", which \"%s\" depends on!\n", mod->dependencies[i], name);
			return -ENOENT;
		}
	}

	// If there's no init function, ignore the module. All modules should have one.
	if (mod->init == NULL)
	{
		module_log("\"%s\" failed to initialize: No init function present, skipping!\n", mod->name);
		return -ENOENT;
	}

	i32 ret = mod->init();
	loaded->loaded = true;
	return ret;
}

void module_register_symbol(const char* name, Elf_Sym* symbol)
{
	hashmap_insert(&module_symbol_map, name, strlen(name), symbol);
}

Elf_Sym* module_get_symbol(const char* name)
{
	Elf_Sym* ret = NULL;

	hashmap_get(&module_symbol_map, ret, name, strlen(name));

	return ret;
}
