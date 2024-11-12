// Kernel error output

#include <menix/common.h>
#include <menix/system/arch.h>
#include <menix/system/elf.h>
#include <menix/system/module.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/thread.h>
#include <menix/util/log.h>
#include <menix/util/self.h>
#include <menix/util/spin.h>

#include <stdarg.h>
#include <stdio.h>

typedef struct ATTR(packed) StackFrame
{
	struct StackFrame* prev;	// The inner frame.
	void* return_addr;			// The address this frame returns to.
} StackFrame;

SpinLock kmesg_lock;

void kmesg(const char* fmt, ...)
{
	spin_acquire_force(&kmesg_lock);

	va_list args;
	va_start(args, fmt);
	vprintf(fmt, args);
	va_end(args);

	spin_free(&kmesg_lock);
}

void ktrace(Context* regs)
{
#ifdef CONFIG_ktrace
	// Write out registers.
	kmesg("Registers:\n");

	Context c;
	if (regs == NULL)
	{
		arch_get_registers(&c);
		regs = &c;
	}
	arch_dump_registers(regs);
	StackFrame* fp = (void*)regs->rbp;
	// Print stack trace.
	kmesg("--- Stack trace (Most recent call first) ---\n");
	for (usize i = 0; i < CONFIG_ktrace_max && fp != NULL; fp = fp->prev, i++)
	{
		// Try to resolve the symbol name and offset.
		const char* name;
		Elf_Sym* sym;

		// If we have found the corresponding symbol, print its name + offset.
		if (module_find_symbol(fp->return_addr, &name, &sym))
			kmesg("\t[%zu]\t0x%p <%s + 0x%zx>\n", i, fp->return_addr, name, fp->return_addr - sym->st_value);
		// If the address is not NULL, but we don't have any matching symbol, just print the address.
		else if (fp->return_addr)
			kmesg("\t[%zu]\t0x%p <\?\?\?>\n", i, fp->return_addr);
	}
	kmesg("--- End of Stack trace ---\n");
#endif
}

ATTR(noreturn) void kabort()
{
	arch_stop(NULL);
	while (1)
		;
}
