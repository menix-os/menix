#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/abi.h>
#include <menix/system/arch.h>
#include <menix/system/archctl.h>
#include <menix/system/sch/process.h>

#include <string.h>

SYSCALL_IMPL(uname, VirtAddr buffer)
{
	// If we have no buffer to write to, fail.
	if (buffer == 0)
		return SYSCALL_ERR(EINVAL);

	struct utsname uname_result;

	fixed_strncpy(uname_result.sysname, "menix");
	// TODO: Get actual network node.
	fixed_strncpy(uname_result.nodename, "localhost");
	fixed_strncpy(uname_result.release, MENIX_RELEASE);
	fixed_strncpy(uname_result.version, MENIX_VERSION);
	fixed_strncpy(uname_result.machine, MENIX_ARCH);

	vm_user_write(arch_current_cpu()->thread->parent, buffer, &uname_result, sizeof(uname_result));

	return SYSCALL_OK(0);
}

// Does architecture specific operations.
SYSCALL_IMPL(archctl, usize operation, usize arg0, usize arg1)
{
	// TODO archctl may fail.
	return SYSCALL_OK(arch_archctl(operation, arg0, arg1));
}

// Performs power control operations.
SYSCALL_STUB(powerctl, usize operation, usize arg0, usize arg1)
