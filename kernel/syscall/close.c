#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/syscall/syscall.h>
#include <menix/thread/process.h>

// Closes a file descriptor.
// `fd`: The file descriptor to close.
SYSCALL_IMPL(close, int fd)
{
	// TODO
	return 0;
}
