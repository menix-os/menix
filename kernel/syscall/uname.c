#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/thread.h>

#include <string.h>

#define fixed_strncpy(dst, src) memcpy(dst, src, MIN(sizeof(dst), sizeof(src)))

SYSCALL_IMPL(uname, VirtAddr buffer)
{
	// If we have no buffer to write to, fail.
	if (buffer == 0)
		return SYSCALL_ERR(EINVAL);

	struct utsname uname_result;

	fixed_strncpy(uname_result.sysname, "Menix");
	// TODO: Get actual network node.
	fixed_strncpy(uname_result.nodename, "localhost");
	fixed_strncpy(uname_result.release, MENIX_RELEASE);
	fixed_strncpy(uname_result.version, MENIX_VERSION);
	fixed_strncpy(uname_result.machine, MENIX_ARCH);

	vm_user_write(arch_current_cpu()->thread->parent, buffer, &uname_result, sizeof(uname_result));

	return SYSCALL_OK(0);
}
