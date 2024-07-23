// Syscall "shutdown"

#include <menix/syscall.h>

#include <errno.h>

SYSCALL_IMPL(shutdown)
{
	// TODO
	return -ENOSYS;
}
