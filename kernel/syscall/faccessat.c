#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/thread.h>

SYSCALL_IMPL(faccessat, int dirfd, VirtAddr path, int mode, int flags)
{
	char* buf = kzalloc(PATH_MAX);
	vm_user_read(arch_current_cpu()->thread->parent, buf, path, PATH_MAX);
	// TODO
	kfree(buf);
	return SYSCALL_OK(0);
}
