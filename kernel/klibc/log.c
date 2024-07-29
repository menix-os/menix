// Kernel error output

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/serial.h>
#include <menix/util/self.h>

#include <stdarg.h>
#include <stdio.h>

void kmesg(const char* fmt, ...)
{
	// TODO: Determine output stream through command line options.
	va_list args;
	va_start(args, fmt);
	vprintf(fmt, args);
	va_end(args);
}

void ktrace()
{
#ifdef CONFIG_ktrace
#ifdef CONFIG_ktrace_registers
	kmesg("Registers:\n");
	arch_dump_registers();
#endif

	const usize size = 32;
	StackFrame* fp;

#ifndef asm_get_frame_pointer
#error "Need asm_get_frame_pointer to do ktrace!"
#else
	asm_get_frame_pointer(fp);
#endif

	// Parse kernel ELF.
	Elf_Hdr* kernel = self_get_kernel();
	void* data = kernel;
	Elf_Shdr* sections = data + kernel->e_shoff;

	// Find symbol and string table.
	Elf_Shdr* strtab = NULL;
	Elf_Shdr* symtab = NULL;
	for (usize i = 0; i < kernel->e_shnum; i++)
	{
		if ((sections + i)->sh_type == SHT_SYMTAB)
		{
			symtab = sections + i;
			break;
		}
	}
	for (usize i = 0; i < kernel->e_shnum; i++)
	{
		// Get the strtab section, but not shstrtab.
		if ((sections + i)->sh_type == SHT_STRTAB && i != kernel->e_shstrndx)
		{
			strtab = sections + i;
			break;
		}
	}
	if (symtab == NULL || strtab == NULL)
		return;
	const char* strtab_data = data + strtab->sh_offset;
	Elf_Sym* symbols = data + symtab->sh_offset;
	const usize symbol_count = symtab->sh_size / symtab->sh_entsize;

	kmesg("Stack trace:\n");
	for (usize i = 0; i < size && fp != NULL; fp = fp->prev, i++)
	{
		const char* symbol_name = NULL;
		usize offset = 0;
		const usize addr = (usize)fp->return_addr;

		// Try to resolve the symbol name and offset.
		for (usize k = 0; k < symbol_count; k++)
		{
			// Check if our address is inside the bounds of the current symobl.
			if (addr >= symbols[k].st_value && addr < (symbols[k].st_value + symbols[k].st_size))
			{
				symbol_name = strtab_data + symbols[k].st_name;
				offset = addr - symbols[k].st_value;
				break;
			}
		}

		if (symbol_name != NULL)
			kmesg("    [%u] 0x%p <%s+0x%x>\n", i, fp->return_addr, symbol_name, offset);
		else
		{
			kmesg("    [%u] 0x%p\n", i, fp->return_addr);
		}
	}
#endif
}
