// Syscalls for process management

#include <menix/fs/vfs.h>
#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/abi.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>

#include <uapi/errno.h>

SYSCALL_IMPL(waitpid, pid_t pid, VirtAddr status, int flags)
{
	// TODO
	return SYSCALL_OK(0);
}
