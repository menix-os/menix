//? Kernel entry point

#include <menix/arch.h>
#include <menix/boot.h>
#include <menix/module.h>
#include <menix/log.h>

void kernel_main()
{
	// Init platform.
	arch_init();

	kmesg(LOG_INFO, "menix v" MENIX_VERSION " (" MENIX_ARCH ")\n");

	// TODO:
	// Initialize modules.
	module_init();

	// Init basic file system.

	// TODO:
	// Call init program.
	// start("/usr/init");

	// Clean up all modules.
	module_fini();

	// TODO:
	// Shut the system down.

	kmesg(LOG_INFO, "shutdown\n");
}
