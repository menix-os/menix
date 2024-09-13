// Module loading/unloading

#include <menix/arch.h>
#include <menix/module.h>
#include <menix/sys/syscall.h>
#include <menix/thread/process.h>

#include <errno.h>

SYSCALL_IMPL(modadd, const char* path)
{
	if (path == NULL)
	{
		proc_errno = -ENOENT;
		return 0;
	}

	return module_load_elf(path);
}

SYSCALL_IMPL(modrem, const char* path)
{
	// TODO
	return 0;
}
