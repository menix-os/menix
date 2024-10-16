// Module loading/unloading

#include <menix/abi/errno.h>
#include <menix/sys/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/module.h>
#include <menix/thread/process.h>

SYSCALL_IMPL(modadd, const char* path)
{
	if (path == NULL)
	{
		thread_errno = -ENOENT;
		return 0;
	}

	return module_load_elf(path);
}

SYSCALL_IMPL(modrem, const char* path)
{
	// TODO
	return 0;
}
