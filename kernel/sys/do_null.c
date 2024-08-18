// Syscall "null"

#include <menix/arch.h>
#include <menix/sys/syscall.h>

#include <errno.h>

// Does nothing.
SYSCALL_IMPL(null)
{
	return -ENOSYS;
}
