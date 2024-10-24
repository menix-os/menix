#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/abi.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>

#include <string.h>

SYSCALL_IMPL(uname, struct utsname* buffer)
{
	// If we have no buffer to write to, fail.
	if (!buffer)
		return -1;

	fixed_strncpy(buffer->sysname, "menix");
	// TODO: Get actual network node.
	fixed_strncpy(buffer->nodename, "localhost");
	fixed_strncpy(buffer->release, CONFIG_release);
	fixed_strncpy(buffer->version, CONFIG_version);
	fixed_strncpy(buffer->machine, CONFIG_arch);
	return 0;
}

// Does architecture specific operations.
SYSCALL_IMPL(archctl, usize operation, usize arg0, usize arg1)
{
	// TODO
	return 0;
}

// Performs power control operations.
SYSCALL_STUB(powerctl, usize operation, usize arg0, usize arg1)
