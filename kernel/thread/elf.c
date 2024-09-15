// ELF parsing utilities.

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/thread/elf.h>

#include <string.h>

void* elf_get_section(void* elf, const char* name)
{
	if (elf == NULL || name == NULL)
		return NULL;

	void* result = NULL;

	// If the ELF is 64-bit.
	if (((u8*)elf)[EI_CLASS] == ELFCLASS64)
	{
		Elf64_Hdr* elf64 = elf;
		Elf64_Shdr* shdr = elf + elf64->e_shoff;
		char* shstrtab = elf + shdr[elf64->e_shstrndx].sh_offset;
		for (usize i = 0; i < elf64->e_shnum; i++)
		{
			const usize name_len = strlen(name);
			const usize sect_len = strlen(shstrtab + shdr[i].sh_name);

			if (name_len != sect_len)
				continue;

			if (memcmp(name, shstrtab + shdr[i].sh_name, MIN(name_len, sect_len)) == 0)
			{
				result = shdr + i;
				break;
			}
		}
	}
	// If the ELF is 32-bit.
	else if (((u8*)elf)[EI_CLASS] == ELFCLASS32)
	{
		Elf32_Hdr* elf32 = elf;
		Elf32_Shdr* shdr = elf + elf32->e_shoff;
		char* shstrtab = elf + shdr[elf32->e_shstrndx].sh_offset;
		for (usize i = 0; i < elf32->e_shnum; i++)
		{
			const usize name_len = strlen(name);
			const usize sect_len = strlen(shstrtab + shdr[i].sh_name);

			if (name_len != sect_len)
				continue;

			if (memcmp(name, shstrtab + shdr[i].sh_name, MIN(name_len, sect_len)) == 0)
			{
				result = shdr + i;
				break;
			}
		}
	}

	return result;
}

bool elf_load(PageMap* page_map, Handle* handle, usize base, ElfInfo* info)
{
	if (handle == NULL)
	{
		elf_log("Failed to load ELF: Could not read the file.\n");
		return false;
	}

	// Read ELF header.
	Elf_Hdr hdr;
	if (handle->read(handle, NULL, &hdr, sizeof(hdr), 0) != sizeof(hdr))
	{
		elf_log("Failed to load ELF: Could not read entire header.\n");
		return false;
	}

	// Verify header magic.
	if (memcmp(hdr.e_ident, ELF_MAG, sizeof(ELF_MAG)) != 0)
	{
		elf_log("Failed to load ELF: File is not an ELF executable.\n");
		return false;
	}

	// Verify ELF identification.
	if (hdr.e_ident[EI_CLASS] != EI_ARCH_CLASS || hdr.e_ident[EI_DATA] != EI_ARCH_DATA ||
		hdr.e_ident[EI_VERSION] != EV_CURRENT || hdr.e_ident[EI_OSABI] != ELFOSABI_SYSV ||
		hdr.e_machine != EI_ARCH_MACHINE || hdr.e_phentsize != sizeof(Elf_Phdr))
	{
		elf_log("Failed to load ELF: File is not designed to run on this machine.\n");
		return false;
	}

	// Evaluate program headers.
	Elf_Phdr phdr;
	for (usize i = 0; i < hdr.e_phnum; i++)
	{
		// Read the current header.
		const usize phdr_off = hdr.e_phoff + (i * sizeof(Elf_Phdr));
		if (handle->read(handle, NULL, &phdr, sizeof(Elf_Phdr), phdr_off) != sizeof(Elf_Phdr))
		{
			elf_log("Failed to load ELF: Could not read program header at offset %zu.\n", phdr_off);
			return false;
		}

		switch (phdr.p_type)
		{
			case PT_LOAD:
			{
				// Calculate protetion
				usize prot = 0;
				if (phdr.p_flags & PF_R)
					prot |= PROT_READ;
				else
					elf_log("Potential bug: Program header %zu does not have read permission.\n");
				if (phdr.p_flags & PF_W)
					prot |= PROT_WRITE;
				if (phdr.p_flags & PF_X)
					prot |= PROT_EXEC;

				usize page_count = ALIGN_UP(phdr.p_memsz, CONFIG_page_size) / CONFIG_page_size;

				// Map memory into the page map.
				if (vm_map(page_map, phdr.p_vaddr + base, page_count * CONFIG_page_size, prot, MAP_FIXED, NULL, 0) ==
					(VirtAddr)MAP_FAILED)
				{
					elf_log("Failed to load ELF: Could not map %zu pages to 0x%p.\n", page_count, phdr.p_vaddr + base);
					return false;
				}

				// Create temporary mapping in the kernel page map so we can write to it.
				void* foreign = vm_map_foreign(page_map, phdr.p_vaddr + base, page_count);
				if (foreign == MAP_FAILED)
				{
					elf_log("Failed to load ELF: Could not map foreign memory region!\n");
					return false;
				}

				// Load data from file.
				handle->read(handle, NULL, foreign, phdr.p_filesz, phdr.p_offset);

				// Zero out the remaining data.
				memset(foreign + phdr.p_filesz, 0, phdr.p_memsz - phdr.p_filesz);

				// Destroy temporary mapping.
				if (vm_unmap_foreign(foreign, page_count) == false)
				{
					elf_log("Failed to load ELF: Could not unmap foreign memory region!\n");
					return false;
				}

				break;
			}
			case PT_INTERP:
			{
				// TODO: Load interpreter at CONFIG_user_interp_base
				break;
			}
		}
	}

	info->entry_point = hdr.e_entry;

	return true;
}
