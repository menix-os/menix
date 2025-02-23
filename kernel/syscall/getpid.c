#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/thread.h>

SYSCALL_IMPL(getpid)
{
	return SYSCALL_OK(arch_current_cpu()->thread->parent->id);
}
