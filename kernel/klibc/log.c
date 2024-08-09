// Kernel error output

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/thread/spin.h>
#include <menix/util/self.h>

#include <stdarg.h>
#include <stdio.h>

static SpinLock lock = spin_new();

void kmesg(const char* fmt, ...)
{
	spin_acquire_force(&lock);

	va_list args;
	va_start(args, fmt);
	vprintf(fmt, args);
	va_end(args);

	spin_free(&lock);
}

void ktrace()
{
#ifdef CONFIG_ktrace
#ifndef asm_get_frame_pointer
#error "Need asm_get_frame_pointer to do ktrace!"
#else
	StackFrame* fp;
	asm_get_frame_pointer(fp);
#endif
#ifdef CONFIG_ktrace_registers
	// Write out registers.
	kmesg("Registers:\n");
	arch_dump_registers();
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

	// Print stack trace.
	kmesg("Stack trace:\n");
	for (usize i = 0; i < CONFIG_ktrace_max && fp != NULL; fp = fp->prev, i++)
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

		// If we have found the corresponding symbol, print its name + offset.
		if (symbol_name != NULL)
			kmesg("    [%u] 0x%p <%s+0x%x>\n", i, fp->return_addr, symbol_name, offset);
		// If the address is not NULL, but we don't have any matching symbol, just print the address.
		else if (fp->return_addr)
		{
			kmesg("    [%u] 0x%p < ??? >\n", i, fp->return_addr);
		}
	}
#endif
}

ATTR(noreturn) void kabort()
{
	arch_stop(NULL);
	while (1)
		;
}
