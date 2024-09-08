// Module loading/unloading

#include <menix/arch.h>
#include <menix/module.h>
#include <menix/sys/syscall.h>
#include <menix/thread/process.h>

SYSCALL_IMPL(modadd, const char* path)
{
	return module_load(path);
}

SYSCALL_IMPL(modrem, const char* path)
{
	// TODO
	return 0;
}
