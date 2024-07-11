//? Kernel entry point

#include <menix/boot.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/syscall.h>

#include <errno.h>

void kernel_main(BootInfo* info)
{
	// Say hello to the console.
	kmesg(LOG_INFO, "menix v" MENIX_VERSION " (" MENIX_ARCH ")\n");

	// Initialize all modules.
	module_init();

	// Init basic file system.

	// TODO: Call init program.
	// exec("/usr/init");

	// Clean up all modules.
	module_fini();

	// TODO: Shut the system down.

	// Say goodbye.
	kmesg(LOG_INFO, "shutdown\n");
}

SYSCALL_IMPL(null)
{
	return -ENOSYS;
}
