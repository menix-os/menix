// uname syscall

#include <menix/common.h>
#include <menix/syscall/syscall.h>
#include <menix/system/abi.h>

#include <string.h>

#include "menix/memory/vm.h"

SYSCALL_IMPL(uname, struct utsname* buffer)
{
	// If we have no buffer to write to, fail.
	if (buffer == NULL)
		return -1;

	vm_user_access({
		fixed_strncpy(buffer->sysname, "menix");
		// TODO: Get actual network node.
		fixed_strncpy(buffer->nodename, "localhost");
		fixed_strncpy(buffer->release, CONFIG_release);
		fixed_strncpy(buffer->version, CONFIG_version);
		fixed_strncpy(buffer->machine, CONFIG_arch);
	});

	return 0;
}
