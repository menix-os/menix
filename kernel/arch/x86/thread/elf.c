// x86-specific ELF handling.

#include <menix/drv/module.h>
#include <menix/fs/vfs.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/thread/elf.h>
#include <menix/util/hash_map.h>

i32 elf_do_reloc(Elf_Rela* reloc, Elf_Sym* symtab_data, const char* strtab_data, Elf_Shdr* section_headers,
				 void* base_virt)
{
	Elf_Sym* symbol = symtab_data + ELF64_R_SYM(reloc->r_info);
	const char* symbol_name = strtab_data + symbol->st_name;

	void* location = base_virt + reloc->r_offset;

	switch (ELF_R_TYPE(reloc->r_info))
	{
		case R_X86_64_64:
		case R_X86_64_GLOB_DAT:
		case R_X86_64_JUMP_SLOT:
		{
			void* resolved;
			if (symbol->st_shndx == 0)
			{
				Elf_Sym resolved_sym = module_get_symbol(symbol_name);
				if (resolved_sym.st_value == 0)
				{
					module_log("Failed to find symbol \"%s\"!\n", symbol_name);
					return 1;
				}
				resolved = (void*)resolved_sym.st_value;
			}
			else
				resolved = base_virt + symbol->st_value;

			*(void**)location = resolved + reloc->r_addend;
			break;
		}
		case R_X86_64_RELATIVE:
		{
			*(void**)location = base_virt + reloc->r_addend;
			break;
		}
		default:
		{
			module_log("Unhandled relocation %zu!\n", ELF_R_TYPE(reloc->r_info));
			return 1;
		}
	}
	return 0;
}
