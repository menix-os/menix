// x86-specific ELF handling.

#include <menix/fs/vfs.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/module.h>
#include <menix/thread/elf.h>
#include <menix/util/hash_map.h>

#include <errno.h>
#include <string.h>

i32 elf_module_load(const char* path)
{
	// Get module handle from file.
	VfsNode* const node = vfs_get_node(vfs_get_root(), path, true);
	if (node == NULL)
	{
		kmesg("No module at \"%s\"!\n", path);
		return -ENOENT;
	}

	i32 ret = 1;
	Handle* const handle = node->handle;

	LoadedModule* loaded = kzalloc(sizeof(LoadedModule));
	strncpy(loaded->file_path, path, sizeof(loaded->file_path));

	// Read ELF header.
	Elf_Hdr* const hdr = kmalloc(sizeof(Elf_Hdr));
	handle->read(handle, NULL, hdr, sizeof(Elf_Hdr), 0);

	// Check magic.
	if (memcmp(hdr->e_ident, ELF_MAG, sizeof(ELF_MAG)) != 0)
	{
		module_log("Module \"%s\" is not an ELF executable!\n", path);
		goto leave;
	}
	// Check rest of the identification fields. x86_64 is 64-bit, little endian.
	if (hdr->e_ident[EI_CLASS] != ELFCLASS64 || hdr->e_ident[EI_DATA] != ELFDATA2LSB ||
		hdr->e_ident[EI_VERSION] != EV_CURRENT || hdr->e_ident[EI_OSABI] != ELFOSABI_SYSV ||
		hdr->e_machine != EM_X86_64)
	{
		module_log("Module \"%s\" is not designed to run on this machine!\n", path);
		goto leave;
	}
	// In order to relocate the kernel module, it needs to actually be relocatable...
	if (hdr->e_type != ET_REL)
	{
		module_log("Module \"%s\" is not a relocatable ELF executable!\n");
		goto leave;
	}

	// Read section headers and load ones with SHF_ALLOC.
	Elf_Shdr* section_headers = kmalloc(sizeof(Elf_Shdr) * hdr->e_shnum);
	handle->read(handle, NULL, section_headers, sizeof(Elf_Hdr) * hdr->e_shnum, hdr->e_shoff);
	for (usize i = 0; i < hdr->e_shnum; i++)
	{
		Elf_Shdr* section = section_headers + i;

		// Load symbol and string tables into memory.
		if (section->sh_type == SHT_SYMTAB || section->sh_type == SHT_STRTAB)
		{
			void* section_data = kmalloc(section->sh_size);
			handle->read(handle, NULL, section_data, section->sh_size, section->sh_offset);
			section->sh_addr = (Elf_Addr)section_data;
		}

		// If the current section has no data to load, skip it.
		if (section->sh_size == 0 || (section->sh_flags & SHF_ALLOC) == 0)
			continue;

		// Allocate enough pages.
		void* data = pm_get_phys_base() + pm_arch_alloc((section->sh_size / CONFIG_page_size) + 1);
		// Keep track of allocated data for unloading.
		loaded->maps[loaded->num_maps].address = data;
		loaded->maps[loaded->num_maps].size = section->sh_size;
		loaded->num_maps++;

		// Data is stored in the fil
		if (section->sh_type == SHT_PROGBITS)
			handle->read(handle, NULL, data, section->sh_size, section->sh_offset);
		// File has no data, instead zero out the buffer.
		if (section->sh_type == SHT_NOBITS)
			memset(data, 0, section->sh_size);

		// Update the section location to the one we allocated.
		section->sh_addr = (Elf_Addr)data;
	}

	const char* shstrtab = (const char*)section_headers[hdr->e_shstrndx].sh_addr;

	// Do relocations.
	for (usize i = 0; i < hdr->e_shnum; i++)
	{
		Elf_Shdr* const section = section_headers + i;

		// Only care about relocations.
		if (section->sh_type != SHT_RELA)
			continue;

		// Check if relocation info size is correct.
		if (section->sh_entsize != sizeof(Elf_Rela))
		{
			module_log("Failed to relocate module \"%s\", sh_entsize doesn't match (= 0x%zu)!\n", path,
					   section->sh_entsize);
			goto reloc_fail;
		}

		// Load relocations from ELF.
		Elf_Rela* relocation_table = kmalloc(section->sh_size);
		handle->read(handle, NULL, relocation_table, section->sh_size, section->sh_offset);
		section->sh_addr = (Elf_Addr)relocation_table;

		// Get the section to relocate.
		Elf_Shdr* target_section = section_headers + section->sh_info;
		// If the target section was not loaded, it doesn't make sense to relocate it.
		if (target_section->sh_addr == 0)
			continue;
		char* target_section_data = (char*)target_section->sh_addr;

		// Get symbol table for this RELA section.
		Elf_Shdr* symbol_table = section_headers + section->sh_link;
		Elf_Sym* symbol_table_data = (Elf_Sym*)symbol_table->sh_addr;

		// Get the string table for this symbol table.
		Elf_Shdr* string_table = section_headers + symbol_table->sh_link;
		const char* string_table_data = (const char*)string_table->sh_addr;

		// Handle relocations.
		for (usize rel = 0; rel < section->sh_size / section->sh_entsize; rel++)
		{
			Elf_Rela* reloc = relocation_table + rel;
			Elf_Sym* symbol = symbol_table_data + ELF64_R_SYM(reloc->r_info);
			const char* symbol_name = string_table_data + symbol->st_name;

			void* location = (void*)(target_section_data + reloc->r_offset);
			usize symbol_value = 0;
			// Check if the symbol is defined in the same file.
			if (symbol->st_shndx > 0)
			{
				// Calculate the location of the symbol
				Elf_Shdr* symbol_section = (section_headers + symbol->st_shndx);
				symbol_value = symbol_section->sh_addr + symbol->st_value + reloc->r_addend;
			}
			else
			{
				Elf_Sym* resolved_sym = module_get_symbol(symbol_name);
				if (resolved_sym == NULL)
				{
					module_log("Failed to find symbol \"%s\"!\n", symbol_name);
					return 1;
				}
				symbol_value = resolved_sym->st_value;
			}

			switch (ELF_R_TYPE(reloc->r_info))
			{
				case R_X86_64_64:
				{
					*(u64*)location = (u64)symbol_value;
					break;
				}
				case R_X86_64_32:
				case R_X86_64_32S:
				{
					*(u32*)location = (u32)symbol_value;
					break;
				}
				default:
				{
					module_log("Unhandled relocation %zu (Relocation No. %zu in \"%s\")!\n", ELF_R_TYPE(reloc->r_info),
							   rel, shstrtab + section->sh_name);
					goto reloc_fail;
				}
			}
		}
	}

	// Correct mappings so not every page is read/write.
	for (usize i = 0; i < hdr->e_shnum; i++)
	{
		const Elf_Shdr* section = section_headers + i;

		// Get only sections with data.
		if (section->sh_size == 0 || (section->sh_flags & SHF_ALLOC) == 0)
			continue;

		usize flags = 0;
		if (section->sh_flags & SHF_WRITE)
			flags |= PAGE_READ_WRITE;

		vm_arch_map_page(NULL, section->sh_addr - (PhysAddr)pm_get_phys_base(), (void*)section->sh_addr, flags);
	}

	// Find .mod section.
	isize mod_index = -1;

	for (usize i = 0; i < hdr->e_shnum; i++)
	{
		const Elf_Shdr* section = section_headers + i;
		const char* section_name = (const char*)(shstrtab + section->sh_name);
		if (strncmp(section_name, ".mod", 4) == 0)
		{
			mod_index = i;
			break;
		}
	}
	if (mod_index == -1)
	{
		module_log("Failed to load module \"%s\", module doesn't contain a .mod section!\n", path);
		goto mod_section_fail;
	}

	// Load all dependencies.

	// Register all symbols.
	isize symtab_index = -1;
	isize strtab_index = -1;
	for (usize i = 0; i < hdr->e_shnum; i++)
	{
		if (strncmp((const char*)(shstrtab + section_headers[i].sh_name), ".symtab", 7) == 0)
			symtab_index = i;
		if (strncmp((const char*)(shstrtab + section_headers[i].sh_name), ".strtab", 7) == 0)
			strtab_index = i;
	}
	if (symtab_index != -1 && strtab_index != -1)
	{
		Elf_Sym* symbols = (Elf_Sym*)section_headers[symtab_index].sh_addr;
		const char* symbol_names = (const char*)section_headers[strtab_index].sh_addr;

		for (usize i = 0; i < section_headers[symtab_index].sh_size / section_headers[symtab_index].sh_entsize; i++)
		{
			if (symbols[i].st_info == (STB_GLOBAL << 4 | STT_FUNC))
				module_register_symbol(symbol_names + (symbols[i].st_name), symbols + i);
		}
	}

	// Register module.
	loaded->module = (Module*)(section_headers[mod_index].sh_addr);
	module_register(loaded);

	// Everything went smoothly, so exit.
	ret = 0;
	goto leave;

mod_section_fail:
reloc_fail:
	kfree(loaded);

leave:
	kfree(hdr);

	return ret;
}
