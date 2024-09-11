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

	for (usize i = 0; i < hdr->e_phnum; i++)
	{
		Elf_Phdr* seg = program_headers + i;
		if (seg->p_type == PT_LOAD)
		{
			// Allocate enough pages.
			PhysAddr data = pm_arch_alloc((seg->p_memsz / CONFIG_page_size) + 1);

			// Update the section location to the one we allocated.
			seg->p_paddr = (Elf_Addr)data;
			seg->p_vaddr = (Elf_Addr)(pm_get_phys_base() + data);

			// Keep track of allocated data for unloading.
			loaded->maps[loaded->num_maps].address = (void*)seg->p_vaddr;
			loaded->maps[loaded->num_maps].size = seg->p_memsz;
			loaded->num_maps++;

			// Read data from file.
			handle->read(handle, NULL, (void*)seg->p_vaddr, seg->p_filesz, seg->p_offset);
			// Zero out unloaded data.
			memset((void*)seg->p_vaddr + seg->p_filesz, 0, seg->p_memsz - seg->p_filesz);
		}
		else if (seg->p_type == PT_DYNAMIC)
		{
			// Handle dynamic table.
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
				}
			}
		}
	}

	// Base address where the first PT_LOAD entry has been loaded at.
	void* base_addr = loaded->maps[0].address;

	// Load string table.
	char* strtab_data = kmalloc(dt_strsz);
	handle->read(handle, NULL, strtab_data, dt_strsz, dt_strtab);

	// Load symbol table.
	const usize dt_symsz = dt_strtab - dt_symtab;
	Elf_Sym* symtab_data = kmalloc(dt_symsz);
	handle->read(handle, NULL, symtab_data, dt_symsz, dt_symtab);

	// Load relocation table.
	Elf_Rela* relocation_data = kmalloc(dt_relasz);
	handle->read(handle, NULL, relocation_data, dt_relasz, dt_rela);

	// Handle relocations.
	for (usize rel = 0; rel < dt_relasz / dt_relaent; rel++)
	{
		Elf_Rela* reloc = relocation_data + rel;
		Elf_Sym* symbol = symtab_data + ELF64_R_SYM(reloc->r_info);
		const char* symbol_name = strtab_data + symbol->st_name;

		void* target_section_data = base_addr + (section_headers[symbol->st_shndx].sh_addr);
		void* location = target_section_data + reloc->r_offset;

		switch (ELF_R_TYPE(reloc->r_info))
		{
			case R_X86_64_64:
			{
				break;
			}
			case R_X86_64_GLOB_DAT:
			case R_X86_64_JUMP_SLOT:
			{
				void* resolved;
				if (symbol->st_shndx == 0)
					resolved = (void*)module_get_symbol(symbol_name)->st_value;
				else
				{
					Elf_Shdr* symbol_section = (section_headers + symbol->st_shndx);
					resolved = base_addr + symbol_section->sh_addr + symbol->st_value + reloc->r_addend;
				}
				if (resolved == NULL)
				{
					module_log("Failed to find symbol \"%s\"!\n", symbol_name);
					return 1;
				}
				*(void**)location = resolved;
				break;
			}
			case R_X86_64_RELATIVE:
			{
				*(void**)location = base_addr + reloc->r_addend;
				break;
			}
			default:
			{
				module_log("Unhandled relocation %zu (Relocation No. %zu in \"%s\")!\n", ELF_R_TYPE(reloc->r_info), rel,
						   path);
				goto reloc_fail;
			}
		}
	}

	// Correct mappings so not every page is read/write.
	for (usize i = 0; i < hdr->e_phnum; i++)
	{
		const Elf_Phdr* segment = program_headers + i;

		// Get only sections with data.
		if ((segment->p_type & PT_LOAD) == 0)
			continue;

		usize flags = 0;
		if (segment->p_flags & PF_W)
			flags |= PAGE_READ_WRITE;
		if ((segment->p_flags & PF_X) == 0)
			flags |= PAGE_EXECUTE_DISABLE;

		vm_arch_map_page(NULL, segment->p_paddr, (void*)segment->p_vaddr, flags);
	}

	// Find .mod section.
	isize mod_index = -1;
	char* shstrndx = kmalloc(section_headers[hdr->e_shstrndx].sh_size);
	handle->read(handle, NULL, shstrndx, section_headers[hdr->e_shstrndx].sh_size,
				 section_headers[hdr->e_shstrndx].sh_offset);
	for (usize i = 0; i < hdr->e_shnum; i++)
	{
		const Elf_Shdr* section = section_headers + i;
		if (strncmp(shstrndx + section->sh_name, ".mod", 4) == 0)
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

	// TODO: Register all symbols.
	// if (symtab_index != -1 && strtab_index != -1)
	//{
	//	Elf_Sym* symbols = (Elf_Sym*)section_headers[symtab_index].sh_addr;
	//	const char* symbol_names = (const char*)section_headers[strtab_index].sh_addr;
	//
	//	for (usize i = 0; i < section_headers[symtab_index].sh_size / section_headers[symtab_index].sh_entsize; i++)
	//	{
	//		if (symbols[i].st_info == (STB_GLOBAL << 4 | STT_FUNC))
	//			module_register_symbol(symbol_names + (symbols[i].st_name), symbols + i);
	//	}
	//}

	// Register module.
	loaded->module = kmalloc(section_headers[mod_index].sh_size);
	handle->read(handle, NULL, loaded->module, section_headers[mod_index].sh_size,
				 section_headers[mod_index].sh_offset);
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
