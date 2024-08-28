// File system syscalls

#include <menix/abi.h>
#include <menix/common.h>
#include <menix/sys/syscall.h>

// Writes data from a buffer to a file descriptor.
// `fd`: The file descriptor to write to.
// `buf`: The data to write.
// `size`: The amount of data to write.
SYSCALL_IMPL(write, u32 fd, const void* buf, usize size)
{
	// TODO
	// Get FileDescriptor from fd number and handle errors.
	// Get handle from the FileDescriptor.
	// Write to the handle.
	return 0;
}

// Reads from a file descriptor to a buffer.
// `fd`: The file descriptor to read from.
// `buf`: A buffer to write to.
// `size`: The amount of data to read.
SYSCALL_IMPL(read, u32 fd, void* buf, usize size)
{
	// TODO
	return 0;
}

// Opens a connection between a file and a file descriptor. Returns a file descriptor.
// `fd`: The file descriptor root.
// `path`: The path to the file to be opened, relative to fd.
// `buf`: A buffer to write to.
// `size`: The amount of data to read.
SYSCALL_IMPL(openat, int fd, const char* path, int oflag, mode_t mode)
{
	// TODO
	return 0;
}

// Opens a connection between a file and a file descriptor. Returns a file descriptor.
// `path`: The path to the file to be opened.
// `buf`: A buffer to write to.
// `size`: The amount of data to read.
SYSCALL_IMPL(open, const char* path, int oflag, mode_t mode)
{
	return syscall_openat(AT_FDCWD, path, oflag, mode);
}

// Closes a file descriptor.
// `fd`: The file descriptor to close.
SYSCALL_IMPL(close, int fd)
{
	// TODO
	return 0;
}

SYSCALL_IMPL(ioctl, u32 fd, u32 request, void* argument)
{
	// TODO
	return 0;
}
