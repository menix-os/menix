#include <menix/common.h>
#include <menix/fs/devtmpfs.h>
#include <menix/system/logger.h>
#include <menix/util/cmd.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

static LoggerWriteFn logger_callbacks[32] = {0};

void logger_register(const char* name, LoggerWriteFn callback)
{
	for (usize i = 0; i < ARRAY_SIZE(logger_callbacks); i++)
	{
		if (logger_callbacks[i] == NULL)
		{
			logger_callbacks[i] = callback;
			print_log("log: Registered new logging sink \"%s\"\n", name);
			return;
		}
	}
	// If we get here, no callback slots are free anymore.
	print_warn("log: Unable to register new callback function, all slots are in use!\n");
}

void logger_write(const char* buf, usize len)
{
	for (usize i = 0; i < ARRAY_SIZE(logger_callbacks); i++)
	{
		if (likely(logger_callbacks[i]))
			logger_callbacks[i](buf, len);
	}
}

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
	usize __time = clock_get_elapsed();
	usize __secs = (__time / 1000000000);
	usize __millis = ((__time / 1000) % 1000000);
	CpuInfo* __cpu = arch_current_cpu();
	usize __tid = 0;
	if (__cpu != NULL && __cpu->thread != NULL)
		__tid = __cpu->thread->id;
	printf("[%5zu.%06zu] [%7zu] ", __secs, __millis, __tid);

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
	{
		// TODO: Convert to define, this only works on x86_64
		asm volatile("int $3");
	}

	arch_dump_registers(regs);

	StackFrame* fp = __builtin_frame_address(0);

	// Print stack trace.
	print_log("--- Stack trace (Most recent call first) ---\n");
	for (usize i = 0; i < 32 && fp != NULL; fp = fp->prev, i++)
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
