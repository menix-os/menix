// ELF parsing utilities.

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/elf.h>
#include <menix/system/module.h>

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
		print_log("elf: Failed to load ELF: Could not read the file.\n");
		return false;
	}

	// Read ELF header.
	Elf_Hdr hdr;
	if (handle->read(handle, NULL, &hdr, sizeof(hdr), 0) != sizeof(hdr))
	{
		print_log("elf: Failed to load ELF: Could not read entire header.\n");
		return false;
	}

	// Verify header magic.
	if (memcmp(hdr.e_ident, ELF_MAG, sizeof(ELF_MAG)) != 0)
	{
		print_log("elf: Failed to load ELF: File is not an ELF executable.\n");
		return false;
	}

	// Verify ELF identification.
	if (hdr.e_ident[EI_CLASS] != EI_ARCH_CLASS || hdr.e_ident[EI_DATA] != EI_ARCH_DATA ||
		hdr.e_ident[EI_VERSION] != EV_CURRENT || hdr.e_ident[EI_OSABI] != ELFOSABI_SYSV ||
		hdr.e_machine != EI_ARCH_MACHINE || hdr.e_phentsize != sizeof(Elf_Phdr))
	{
		print_log("elf: Failed to load ELF: File is not designed to run on this machine.\n");
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
			print_log("elf: Failed to load ELF: Could not read program header at offset %zu.\n", phdr_off);
			return false;
		}

		switch (phdr.p_type)
		{
			case PT_LOAD:
			{
				// Calculate protetion
				VMProt prot = 0;
				if (phdr.p_flags & PF_R)
					prot |= VMProt_Read;
				else
					print_log("elf: Potential bug: Program header %zu does not have read permission.\n");
				if (phdr.p_flags & PF_W)
					prot |= VMProt_Write;
				if (phdr.p_flags & PF_X)
					prot |= VMProt_Execute;

				const usize page_size = vm_get_page_size(VMLevel_0);

				// Align the virtual address for mapping.
				VirtAddr aligned_virt = ALIGN_DOWN(phdr.p_vaddr + base, page_size);
				usize align_difference = phdr.p_vaddr + base - aligned_virt;

				// Amount of pages to allocate for this segment.
				const usize page_count = ALIGN_UP(phdr.p_memsz + align_difference, page_size) / page_size;

				// Map the physical pages to the requested address.
				for (usize p = 0; p < page_count; p++)
				{
					PhysAddr page = pm_alloc(1);
					if (vm_map(page_map, page, (VirtAddr)(aligned_virt + (p * page_size)), prot, VMFlags_User,
							   VMLevel_0) == false)
					{
						print_log("elf: Failed to load ELF: Could not map %zu pages to 0x%p.\n", page_count, aligned_virt);
						return false;
					}
				}

				// Create temporary mapping in the kernel page map so we can write to it.
				void* foreign = vm_map_foreign(page_map, aligned_virt, page_count);
				if (foreign == (void*)~0UL)
				{
					print_log("elf: Failed to load ELF: Could not map foreign memory region!\n");
					return false;
				}

				// Load data from file.
				handle->read(handle, NULL, foreign + align_difference, phdr.p_filesz, phdr.p_offset);

				// Zero out the remaining data.
				memset(foreign + align_difference + phdr.p_filesz, 0, phdr.p_memsz - phdr.p_filesz);

				// Destroy temporary mapping.
				if (vm_unmap_foreign(foreign, page_count) == false)
				{
					print_log("elf: Failed to load ELF: Could not unmap foreign memory region!\n");
					return false;
				}

				break;
			}
			case PT_PHDR:
			{
				info->at_phdr = base + phdr.p_vaddr;
				break;
			}
			case PT_INTERP:
			{
				info->ld_path = kzalloc(phdr.p_filesz + 1);
				handle->read(handle, NULL, info->ld_path, phdr.p_filesz, phdr.p_offset);
				break;
			}
		}
	}

	info->at_entry = base + hdr.e_entry;
	info->at_phnum = hdr.e_phnum;
	info->at_phent = hdr.e_phentsize;

	return true;
}

i32 elf_do_reloc(Elf_Rela* reloc, Elf_Sym* symtab_data, const char* strtab_data, Elf_Shdr* section_headers,
				 void* base_virt)
{
	Elf_Sym* symbol = symtab_data + ELF64_R_SYM(reloc->r_info);
	const char* symbol_name = strtab_data + symbol->st_name;

	void* location = base_virt + reloc->r_offset;

	switch (ELF_R_TYPE(reloc->r_info))
	{
#if defined(CONFIG_arch_x86_64)
		case R_X86_64_64:
		case R_X86_64_GLOB_DAT:
		case R_X86_64_JUMP_SLOT:
#elif defined(CONFIG_arch_aarch64)
#elif defined(CONFIG_arch_riscv64)
		case R_RISCV_64:
		case R_RISCV_JUMP_SLOT:
#elif defined(CONFIG_arch_loongarch64)
#endif
		{
			void* resolved;
			if (symbol->st_shndx == 0)
			{
				Elf_Sym resolved_sym = module_get_symbol(symbol_name);
				if (resolved_sym.st_value == 0)
				{
					kassert(false, "Failed to find symbol \"%s\"!\n", symbol_name);
					return 1;
				}
				resolved = (void*)resolved_sym.st_value;
			}
			else
				resolved = base_virt + symbol->st_value;

			*(void**)location = resolved + reloc->r_addend;
			break;
		}
#if defined(CONFIG_arch_x86_64)
		case R_X86_64_RELATIVE:
#elif defined(CONFIG_arch_aarch64)
#elif defined(CONFIG_arch_riscv64)
		case R_RISCV_RELATIVE:
#elif defined(CONFIG_arch_loongarch64)
#endif
		{
			*(void**)location = base_virt + reloc->r_addend;
			break;
		}
		default:
		{
			kassert(false, "Unhandled relocation %zu!\n", ELF_R_TYPE(reloc->r_info));
			return 1;
		}
	}
	return 0;
}
