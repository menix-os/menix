// Module and sub-system initialization.

#include <menix/common.h>
#include <menix/fs/vfs.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/elf.h>
#include <menix/system/module.h>
#include <menix/system/pci/pci.h>
#include <menix/util/cmd.h>
#include <menix/util/hash_map.h>
#include <menix/util/log.h>
#include <menix/util/self.h>
#include <menix/util/spin.h>

#include <uapi/errno.h>

#include <string.h>

// We need to see the location and size of the .mod section.
SECTION_DECLARE_SYMBOLS(mod)

// Stores all loaded modules.
static HashMap(LoadedModule*) module_map;

// Stores all known symbols.
static HashMap(Elf_Sym) module_symbol_map;

// Callbacks to run after all modules have been initialized.
static List(ModulePostFn) module_post_fns;

// Virtual memory which is used to map modules into memory.
static VirtAddr module_map_region = VM_MODULE_BASE;
static SpinLock module_map_lock = {0};

void module_init()
{
	// Initialize subsystems.

	// Initialize the module map.
	hashmap_init(module_map, 128);

	u8* module_ptr = (u8*)SECTION_START(mod);

	// Register all built-in modules.
	while (module_ptr < SECTION_END(mod))
	{
		Module* const module = (Module*)module_ptr;
		LoadedModule* module_info = kzalloc(sizeof(LoadedModule));
		module_info->module = module;

		// If this module was explicitly turned off via cmdline, don't register it.
		if (cmd_get_usize(module->name, 1))
			module_register(module->name, module_info);

		// Go to the next module. This contains the dependency string table.
		module_ptr += sizeof(Module) + (sizeof(module->dependencies[0]) * module->num_dependencies);
	}

	// Load modules from VFS.
	const char* dyn_modules_path = cmd_get_str("modules", "/boot/modules");
	VfsNode* module_path = vfs_get_node(vfs_get_root(), dyn_modules_path, true);
	if (module_path == NULL)
	{
		print_warn("module: Can't find dynamic modules, directory \"%s\" is missing!\n", dyn_modules_path);
		goto skip_dynamic;
	}
	if (!S_ISDIR(module_path->handle->stat.st_mode))
	{
		print_warn("module: Can't find dynamic modules, \"%s\" is not a directory!\n", dyn_modules_path);
		goto skip_dynamic;
	}

	// Find all module files in the directory.
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

			// If this module was explicitly turned off via cmdline, don't register it.
			if (cmd_get_usize(node->name, 1) == 0)
				continue;

			// Load only the file's path into the meta info.
			char full_path[256];
			vfs_get_path(node, full_path, 256);
			LoadedModule* module_info = kzalloc(sizeof(LoadedModule));
			strncpy(module_info->file_path, full_path, sizeof(module_info->file_path));
			module_register(node->name, module_info);
		}
	}

	// After registering them all, load the ELFs.
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

			char full_path[256];
			vfs_get_path(node, full_path, 256);
			module_load_elf(full_path);
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
				print_error("module: \"%s\" failed to initialize with error code %i!\n", loaded->module->name, ret);
		}
	}

	// After all modules have been loaded, run over all post-init callbacks registered by the modules.
	for (usize i = 0; i < module_post_fns.length; i++)
	{
		module_post_fns.items[i]();
	}
	list_free(&module_post_fns);
}

void module_fini()
{
	// Clean up all modules.
	for (usize b = 0; b < module_map.capacity; b++)
	{
		if (module_map.buckets == NULL)
			break;

		auto bucket = module_map.buckets + b;
		for (usize i = 0; i < bucket->count; i++)
		{
			LoadedModule* loaded = bucket->items[i].item;
			if (loaded->module->exit)
				loaded->module->exit();
		}
	}
}

LoadedModule* module_get(const char* name)
{
	LoadedModule* get = NULL;
	hashmap_get(&module_map, get, name, strlen(name));
	return get;
}

void module_register(const char* name, LoadedModule* module)
{
	LoadedModule* old;
	if (hashmap_get(&module_map, old, name, strlen(name)))
	{
		// If the module is already loaded, ignore it.
		print_warn("module: Ignoring already loaded module \"%s\" (%s)\n", name,
				   old->file_path[0] != '\0' ? old->file_path : "built-in");
	}
	else
	{
		hashmap_insert(&module_map, name, strlen(name), module);
		module_log("Registered new module \"%s\"\n", name);
	}
}

// Loads a previously registered module.
i32 module_load(const char* name)
{
	LoadedModule* loaded;

	// Check if module is registered.
	if (!hashmap_get(&module_map, loaded, name, strlen(name)))
	{
		print_error("module: Unable to load \"%s\": Not previously registered!\n", name);
		return ENOENT;
	}
	if (loaded->loaded)
		return 0;

	Module* const mod = loaded->module;
	module_log("Loading module at 0x%p: \"%s\", \"%s\"\n", loaded->maps[0].address, mod->name, mod->description);

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

	const i32 ret = mod->init();
	loaded->loaded = true;
	return ret;
}

void module_register_post(ModulePostFn callback)
{
	list_push(&module_post_fns, callback);
}

void module_register_symbol(const char* name, Elf_Sym symbol)
{
	// If the symbol hasn't been registered yet, do so now.
	if (!hashmap_get(&module_symbol_map, symbol, name, strlen(name)))
		hashmap_insert(&module_symbol_map, name, strlen(name), symbol);
}

Elf_Sym module_get_symbol(const char* name)
{
	Elf_Sym ret = {0};

	hashmap_get(&module_symbol_map, ret, name, strlen(name));

	return ret;
}

void module_load_kernel_syms(void* kernel_elf)
{
	// Add all kernel symbols to the symbol map.
	void* const kernel = kernel_elf;
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
			module_register_symbol(symbol_name, symtab_data[sym]);
	}
}

bool module_find_symbol(void* addr, const char** out_name, Elf_Sym** out_symbol)
{
	for (usize b = 0; b < module_symbol_map.capacity; b++)
	{
		if (module_symbol_map.buckets == NULL)
			break;

		auto bucket = module_symbol_map.buckets + b;
		for (usize i = 0; i < bucket->count; i++)
		{
			Elf_Sym* symbol = &bucket->items[i].item;
			// Check if our address is inside the bounds of the current symobl.
			if (addr >= (void*)symbol->st_value && addr < (void*)(symbol->st_value + symbol->st_size))
			{
				*out_name = (const char*)bucket->items[i].key_data;
				*out_symbol = symbol;
				return true;
			}
		}
	}
	return false;
}

i32 module_load_elf(const char* path)
{
	// Get module handle from file.
	VfsNode* const node = vfs_get_node(vfs_get_root(), path, true);
	if (node == NULL)
	{
		module_log("No module at \"%s\"!\n", path);
		return -ENOENT;
	}

	i32 ret = 1;
	Handle* const handle = node->handle;

	LoadedModule* loaded = module_get(node->name);
	if (loaded == NULL)
	{
		module_log("Module \"%s\" was not registered!\n", path);
		return 1;
	}
	if (loaded->module != NULL)
	{
		module_log("Module \"%s\" was already loaded!\n", path);
		return 1;
	}

	// Read ELF header.
	Elf_Hdr* const hdr = kmalloc(sizeof(Elf_Hdr));
	handle->read(handle, NULL, hdr, sizeof(Elf_Hdr), 0);

	// Check magic.
	if (memcmp(hdr->e_ident, ELF_MAG, sizeof(ELF_MAG)) != 0)
	{
		module_log("Module \"%s\" is not an ELF executable!\n", path);
		goto leave;
	}

	// Check rest of the identification fields. All of these need to match to be usable for us.
	if (hdr->e_ident[EI_CLASS] != EI_ARCH_CLASS || hdr->e_ident[EI_DATA] != EI_ARCH_DATA ||
		hdr->e_ident[EI_VERSION] != EV_CURRENT || hdr->e_ident[EI_OSABI] != ELFOSABI_SYSV ||
		hdr->e_machine != EI_ARCH_MACHINE)
	{
		module_log("Module \"%s\" is not designed to run on this machine!\n", path);
		goto leave;
	}

	// In order to relocate the kernel module, it needs to actually be relocatable...
	if (hdr->e_type != ET_DYN)
	{
		module_log("Module \"%s\" is not a dynamic ELF executable!\n", path);
		goto leave;
	}

	// Read program headers.
	Elf_Phdr* program_headers = kmalloc(sizeof(Elf_Phdr) * hdr->e_phnum);
	handle->read(handle, NULL, program_headers, sizeof(Elf_Phdr) * hdr->e_phnum, hdr->e_phoff);

	// Read section headers.
	Elf_Shdr* section_headers = kmalloc(sizeof(Elf_Shdr) * hdr->e_shnum);
	handle->read(handle, NULL, section_headers, sizeof(Elf_Shdr) * hdr->e_shnum, hdr->e_shoff);

	// Variables read from the dynamic table.
	usize dt_strtab = 0;
	isize dt_strsz = 0;
	usize dt_symtab = 0;
	usize dt_rela = 0;
	isize dt_relasz = 0;
	isize dt_relaent = 0;
	isize dt_pltrelsz = 0;
	usize dt_jmprel = 0;
	usize dt_init_array = 0;
	isize dt_init_arraysz = 0;

	// Base address where the first mapping was created.
	void* base_virt = 0;

	for (usize i = 0; i < hdr->e_phnum; i++)
	{
		Elf_Phdr* seg = program_headers + i;

		if (seg->p_type == PT_LOAD)
		{
			spin_lock(&module_map_lock);

			// Amount of pages to allocate for this segment.
			const usize page_size = vm_get_page_size(VMLevel_Small);

			// If not set previously, set the address of the first mapping as the load base.
			if (base_virt == 0)
				base_virt = (void*)module_map_region;

			// Align the virtual address for mapping.
			VirtAddr aligned_virt = ALIGN_DOWN(seg->p_vaddr, page_size);
			// If the aligned address was unaligned, that means the original addr is inbetween two pages.
			// Allocate one more.
			if (aligned_virt < seg->p_vaddr)
				seg->p_memsz += page_size - (seg->p_memsz % page_size);

			PhysAddr pages = pm_alloc((seg->p_memsz / page_size) + 1);

			// Map the physical pages to the requested address.
			for (usize page = 0; page <= seg->p_memsz; page += page_size)
			{
				vm_map(vm_kernel_map, pages + page, (VirtAddr)(base_virt + aligned_virt + page),
					   VMProt_Read | VMProt_Write, 0, VMLevel_Small);

				module_map_region += page_size;
			}

			// Relocate the segment addresses.
			seg->p_vaddr = (Elf_Addr)(base_virt + seg->p_vaddr);

			// Keep track of allocated data for unloading.
			loaded->maps[loaded->num_maps].address = (void*)seg->p_vaddr;
			loaded->maps[loaded->num_maps].size = seg->p_memsz;
			loaded->num_maps++;

			// Read data from file.
			handle->read(handle, NULL, (void*)seg->p_vaddr, seg->p_filesz, seg->p_offset);

			// Zero out unloaded data.
			memset((void*)seg->p_vaddr + seg->p_filesz, 0, seg->p_memsz - seg->p_filesz);

			spin_unlock(&module_map_lock);
		}
		else if (seg->p_type == PT_DYNAMIC)
		{
			// Extract the dynamic table information.
			Elf_Dyn* dynamic_table = kmalloc(seg->p_memsz);
			handle->read(handle, NULL, dynamic_table, seg->p_filesz, seg->p_offset);

			for (usize i = 0; i < seg->p_memsz / sizeof(Elf_Dyn); i++)
			{
				switch (dynamic_table[i].d_tag)
				{
					case DT_STRTAB: dt_strtab = dynamic_table[i].d_un.d_ptr; break;
					case DT_SYMTAB: dt_symtab = dynamic_table[i].d_un.d_ptr; break;
					case DT_STRSZ: dt_strsz = dynamic_table[i].d_un.d_val; break;
					case DT_RELA: dt_rela = dynamic_table[i].d_un.d_ptr; break;
					case DT_RELASZ: dt_relasz = dynamic_table[i].d_un.d_val; break;
					case DT_RELAENT: dt_relaent = dynamic_table[i].d_un.d_val; break;
					case DT_PLTRELSZ: dt_pltrelsz = dynamic_table[i].d_un.d_val; break;
					case DT_JMPREL: dt_jmprel = dynamic_table[i].d_un.d_ptr; break;
					case DT_INIT_ARRAY: dt_init_array = dynamic_table[i].d_un.d_ptr; break;
					case DT_INIT_ARRAYSZ: dt_init_arraysz = dynamic_table[i].d_un.d_val; break;
				}
			}

			kfree(dynamic_table);

			// Sanity check.
			if (dt_strtab == 0 || dt_symtab == 0 || dt_strsz == 0 || dt_rela == 0 || dt_relasz == 0 || dt_relaent == 0)
			{
				module_log("Failed to load module \"%s\": Dynamic section is malformed!\n", path);
				goto reloc_fail;
			}
		}
	}

	module_log("Relocated module \"%s\"\n", path);

	// Load section string table.
	char* shstrtab_data = kmalloc(section_headers[hdr->e_shstrndx].sh_size);
	handle->read(handle, NULL, shstrtab_data, section_headers[hdr->e_shstrndx].sh_size,
				 section_headers[hdr->e_shstrndx].sh_offset);

	// Update the section vaddrs and get the section header containing the module metadata.
	Module* module = 0;
	for (usize i = 0; i < hdr->e_shnum; i++)
	{
		section_headers[i].sh_addr = (Elf_Addr)base_virt + section_headers[i].sh_addr;
		if (strncmp(".mod", shstrtab_data + section_headers[i].sh_name, 4) == 0)
			module = (Module*)section_headers[i].sh_addr;
	}

	// Check if the module information was found.
	if (module == NULL)
	{
		module_log("Failed to load module \"%s\": Module does not contain a .mod section!\n", path);
		goto reloc_fail;
	}

	// Before we do relocations, we have to handle module dependencies, otherwise those symbol resolutions will fail.
	for (usize i = 0; i < module->num_dependencies; i++)
	{
		// Try to find the dependency.
		LoadedModule* loaded_dep = module_get(module->dependencies[i]);
		if (loaded_dep == NULL)
		{
			module_log("Unable to find dynamic dependency \"%64s\" of module \"%s\"!\n", module->dependencies[i],
					   module->name);
			goto reloc_fail;
		}

		// If the dependency exists and is dynamic, load it before this module.
		if (strlen(loaded_dep->file_path) > 0)
			module_load_elf(loaded_dep->file_path);
	}

	// Load string table.
	char* strtab_data = kmalloc(dt_strsz);
	handle->read(handle, NULL, strtab_data, dt_strsz, dt_strtab);

	// Load symbol table.
	const usize dt_symsz = dt_strtab - dt_symtab;
	Elf_Sym* symtab_data = kmalloc(dt_symsz);
	handle->read(handle, NULL, symtab_data, dt_symsz, dt_symtab);

	// Handle relocations for .rela.dyn
	Elf_Rela* relocation_data = kmalloc(dt_relasz);
	handle->read(handle, NULL, relocation_data, dt_relasz, dt_rela);
	for (usize rel = 0; rel < dt_relasz / sizeof(Elf_Rela); rel++)
	{
		if (elf_do_reloc(relocation_data + rel, symtab_data, strtab_data, section_headers, base_virt) != 0)
			goto reloc_fail;
	}
	kfree(relocation_data);

	// Handle relocations for .rela.plt
	relocation_data = kmalloc(dt_pltrelsz);
	handle->read(handle, NULL, relocation_data, dt_pltrelsz, dt_jmprel);
	for (usize rel = 0; rel < dt_pltrelsz / sizeof(Elf_Rela); rel++)
	{
		if (elf_do_reloc(relocation_data + rel, symtab_data, strtab_data, section_headers, base_virt) != 0)
			goto reloc_fail;
	}
	kfree(relocation_data);

	// Correct mappings so not every page is read/write.
	for (usize i = 0; i < hdr->e_phnum; i++)
	{
		const Elf_Phdr* segment = program_headers + i;

		// Get only sections with data.
		if (segment->p_type != PT_LOAD)
			continue;

		VMProt prot = VMProt_Read;
		if (segment->p_flags & PF_W)
			prot |= VMProt_Write;
		if (segment->p_flags & PF_X)
			prot |= VMProt_Execute;

		const usize seg_size = ALIGN_UP(segment->p_memsz, vm_get_page_size(VMLevel_Small));
		for (usize page = 0; page <= seg_size; page += vm_get_page_size(VMLevel_Small))
			vm_protect(vm_kernel_map, segment->p_vaddr + page, prot, 0);
	}

	// Register all global symbols.
	for (usize i = 0; i < dt_symsz / sizeof(Elf_Sym); i++)
	{
		if ((symtab_data[i].st_info & (STB_GLOBAL << 4)) && symtab_data[i].st_size != 0)
		{
			symtab_data[i].st_value += (VirtAddr)base_virt;
			module_register_symbol(strtab_data + symtab_data[i].st_name, symtab_data[i]);
		}
	}

	// Register module.
	loaded->module = module;

	// At this point, everything should be loaded correctly. If we have an init array, we call each function from it.
	void (**init_array)() = base_virt + dt_init_array;
	for (usize i = 0; i < dt_init_arraysz / sizeof(void*); i++)
	{
		if (init_array[i] != NULL)
			init_array[i]();
	}

	// Everything went smoothly, so exit.
	ret = 0;
	kfree(shstrtab_data);
	kfree(strtab_data);
	kfree(symtab_data);
	kfree(program_headers);
	kfree(section_headers);
	goto leave;

reloc_fail:
	for (usize i = 0; i < loaded->num_maps; i++)
	{
		for (usize page = 0; page < loaded->maps[i].size; page += arch_page_size)
			vm_unmap(vm_kernel_map, (VirtAddr)loaded->maps[i].address + page);
	}

leave:
	kfree(hdr);

	return ret;
}
