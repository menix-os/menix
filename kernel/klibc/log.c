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

void kmesg_direct(const char* fmt, ...)
{
	spin_lock(&kmesg_lock);

	va_list args;
	va_start(args, fmt);
	vprintf(fmt, args);
	va_end(args);

	spin_unlock(&kmesg_lock);
}

void ktrace(Context* regs)
{
	if (regs == NULL)
		return;

	arch_dump_registers(regs);
	StackFrame* fp = (void*)regs->rbp;
	// Print stack trace.
	print_log("--- Stack trace (Most recent call first) ---\n");
	for (usize i = 0; i < CONFIG_ktrace_max && fp != NULL; fp = fp->prev, i++)
	{
		// Try to resolve the symbol name and offset.
		const char* name;
		Elf_Sym* sym;

		// If we have found the corresponding symbol, print its name + offset.
		if (module_find_symbol(fp->return_addr, &name, &sym))
			print_log("\t[%zu]\t0x%p <%s + 0x%zx>\n", i, fp->return_addr, name, fp->return_addr - sym->st_value);
		// If the address is not NULL, but we don't have any matching symbol, just print the address.
		else if (fp->return_addr)
			print_log("\t[%zu]\t0x%p <\?\?\?>\n", i, fp->return_addr);
	}
	print_log("--- End of Stack trace ---\n");
}

ATTR(noreturn) void kabort()
{
	arch_stop();
}
