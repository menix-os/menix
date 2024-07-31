// Syscall "null"
// Does nothing.

#include <menix/arch.h>
#include <menix/syscall.h>

#include <errno.h>

SYSCALL_IMPL(null)
{
#ifndef NDEBUG
	kmesg("Hello!\n");
#endif

	return -ENOSYS;
}
