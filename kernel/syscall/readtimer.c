#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/abi.h>
#include <menix/system/arch.h>
#include <menix/system/archctl.h>
#include <menix/system/sch/process.h>
#include <menix/system/time/clock.h>

SYSCALL_IMPL(readtimer, usize clock, VirtAddr time)
{
	const usize val = clock_get_elapsed_ns();
	struct timespec ts = {.tv_nsec = val, .tv_sec = val / 1000000000ULL};
	vm_user_write(arch_current_cpu()->thread->parent, time, &ts, sizeof(ts));
	return SYSCALL_OK(val);
}
