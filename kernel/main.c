//? Kernel entry point

#include <menix/arch.h>
#include <menix/module.h>
#include <menix/stdio.h>

void kernel_main()
{
	// Init platform.
	arch_init();

	printf("menix v" MENIX_VERSION " (" MENIX_ARCH ")\n");

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

	printf("shutdown\n");
}
