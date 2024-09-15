// Kernel error output

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/thread/elf.h>
#include <menix/thread/spin.h>
#include <menix/util/self.h>

#include <stdarg.h>
#include <stdio.h>

void kmesg(const char* fmt, ...)
{
	va_list args;
	va_start(args, fmt);
	vprintf(fmt, args);
	va_end(args);
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
	CpuRegisters regs;
	arch_get_registers(&regs);
	arch_dump_registers(&regs);
#endif

	// Print stack trace.
	kmesg("Stack trace:\n");
	for (usize i = 0; i < CONFIG_ktrace_max && fp != NULL; fp = fp->prev, i++)
	{
		// Try to resolve the symbol name and offset.
		const char* name;
		Elf_Sym* sym;
		if (module_find_symbol(fp->return_addr, &name, &sym))
		{
			// If we have found the corresponding symbol, print its name + offset.
			kmesg("    [%zu] 0x%p <%s+0x%x>\n", i, fp->return_addr, name, sym->st_value);
		}
		// If the address is not NULL, but we don't have any matching symbol, just print the address.
		else if (fp->return_addr)
		{
			kmesg("    [%zu] 0x%p < ??? >\n", i, fp->return_addr);
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
