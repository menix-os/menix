#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>

SYSCALL_IMPL(savetls, VirtAddr addr)
{
#if defined(__x86_64__)
	asm_wrmsr(MSR_FS_BASE, addr);
#endif
	return SYSCALL_OK(0);
}
