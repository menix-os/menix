/*----------------
Kernel entry point
----------------*/

#include <menix/stdio.h>
#include <menix/arch.h>
#include <menix/module.h>

#include <menix/config.h>

void kernel_main(void)
{
	// Init platform.
	arch_init();

	printf("menix v" MENIX_VERSION " (" MENIX_ARCH ")" "\n");

#if CFG_ENABLED(example)
	hello_world_say_hello();
#endif

	// TODO:
	// Init basic file system.
	// Load modules.
	// Initialize drivers.

	// TODO:
	// Call init program.
	// start("/usr/init");

	printf("shutdown\n");

	// TODO:
	// Shut the system down.
}
