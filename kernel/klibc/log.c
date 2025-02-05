// Kernel error output

#include <menix/common.h>
#include <menix/system/arch.h>
#include <menix/system/elf.h>
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
		return;

	arch_dump_registers(regs);

	StackFrame* fp =
#if defined(__x86_64__)
		(void*)regs->rbp;
#elif defined(__riscv) && (__riscv_xlen == 64)
		(void*)regs->x2;
#endif

	// Print stack trace.
	print_log("--- Stack trace (Most recent call first) ---\n");
	for (usize i = 0; i < 32 && fp != NULL; fp = fp->prev, i++)
	{
		print_log("\t[%zu]\t0x%p <\?\?\?>\n", i, fp->return_addr);
	}
	print_log("--- End of Stack trace ---\n");
}

ATTR(noreturn) void kabort()
{
	arch_stop();
}
