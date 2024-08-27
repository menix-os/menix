// Module and sub-system initialization.

#include <menix/common.h>
#include <menix/drv/pci/pci.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/thread/elf.h>
#include <menix/util/hash_map.h>
#include <menix/util/self.h>

#include <errno.h>
#include <string.h>

#ifdef CONFIG_acpi
#include <menix/drv/acpi/acpi.h>
#endif

// We need to see the location and size of the .mod section.
SECTION_DECLARE_SYMBOLS(mod)

static HashMap(Module*) dynamic_modules;

void module_init(BootInfo* info)
{
	hashmap_init(dynamic_modules, 128);

	// Check if the .mod section size is sane.
	if (SECTION_SIZE(mod) % sizeof(Module) != 0)
	{
		kmesg("Ignoring built-in modules: The .mod section has a bogus size of 0x%zx!\n", SECTION_SIZE(mod));
		return;
	}

	// Calculate the module count.
	const usize module_count = SECTION_SIZE(mod) / sizeof(Module);
	const Module* modules = (Module*)SECTION_START(mod);

	// Initialize all built-in modules.
	kmesg("Loading %zu built-in modules.\n", module_count);
	for (usize i = 0; i < module_count; i++)
	{
		kmesg("Loading built-in module \"%s\": %s (Author: %s, License: %s)\n", modules[i].name, modules[i].description,
			  modules[i].author, modules[i].license);

		// If there's no init function, ignore the module. All modules should have one.
		if (modules[i].init == NULL)
		{
			kmesg("\"%s\" failed to initialize: No init function present, skipping!\n", modules[i].name);
			continue;
		}

		const i32 ret = modules[i].init();
		if (ret != 0)
			kmesg("\"%s\" failed to initialize with error code %i!\n", modules[i].name, ret);
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
			kmesg("Unloading \"%s\"\n", modules[i].name);
			modules[i].exit();
		}
	}
}

// TODO: Use a path instead of buffer. Needs a FS.
i32 module_load(const char* path, usize len)
{
	if (path == NULL)
		return -ENOENT;

	Elf_Hdr* const module = (Elf_Hdr*)path;
	void* module_data = module;
	Elf_Shdr* const module_sections = module_data + module->e_shoff;
	// Get string table for symbol names.
	const char* module_strtab = module_data + ((Elf_Shdr*)elf_get_section(module, ".strtab"))->sh_offset;

	Elf_Hdr* const kernel = self_get_kernel();
	void* kernel_data = kernel;
	const char* kernel_strtab = kernel_data + ((Elf_Shdr*)elf_get_section(kernel, ".strtab"))->sh_offset;

	// Note: Kernel modules are built as relocatable executables. This means they don't have Phdrs.
	// Loop over all section headers and check for SHT_REL or SHT_RELA entries. If found, patch them.
	for (usize sect = 0; sect < module->e_shnum; sect++)
	{
		if (module_sections[sect].sh_type != SHT_RELA)
			continue;
		// TODO
	}

	// Finally, get the module metadata and call init.
	Module* mod = module_data + ((Elf_Shdr*)elf_get_section(module, ".mod"))->sh_offset;

	kmesg("Loading dynamic module \"%s\": %s (Author: %s, License: %s)\n", mod->name, mod->description, mod->author,
		  mod->license);

	// If there's no init function, ignore the module. All modules should have one.
	if (mod->init == NULL)
	{
		kmesg("\"%s\" failed to initialize: No init function present, skipping!\n", mod->name);
		return -ENOENT;
	}

	// Call the init function. If it succeeded, register it.
	i32 const ret = mod->init();
	if (ret == 0)
		hashmap_insert(&dynamic_modules, mod->name, strlen(mod->name), mod);

	return ret;
}

void module_unload(const char* name)
{
}
