// Syscall "null"

#include <menix/arch.h>
#include <menix/sys/syscall.h>

#include <errno.h>

// Does nothing.
SYSCALL_IMPL(null)
{
#ifndef NDEBUG
	kmesg("Hello!\n");
#endif

	return -ENOSYS;
}
