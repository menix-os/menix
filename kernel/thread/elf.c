// ELF parsing utilities.

#include <menix/common.h>
#include <menix/thread/elf.h>

#include <string.h>

static Elf_Hdr* self_kernel_addr = NULL;

void elf_set_kernel(Elf_Hdr* addr)
{
	self_kernel_addr = addr;
}

Elf_Hdr* elf_get_kernel()
{
	return self_kernel_addr;
}

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
