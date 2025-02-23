#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/thread.h>

SYSCALL_IMPL(gettid)
{
	const usize val = arch_current_cpu()->thread->id;
	return SYSCALL_OK(val);
}
