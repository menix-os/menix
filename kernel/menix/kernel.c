/*----------------
Kernel entry point
----------------*/

#include <menix/arch.h>
#include <menix/config.h>
#include <menix/drv/driver.h>
#include <menix/stdio.h>

void kernel_main(void)
{
	// Init platform.
	arch_init();

	printf("menix v" MENIX_VERSION " (" MENIX_ARCH ")\n");
	// TODO:
	// Initialize drivers.
	drv_init();

	// Init basic file system.

	// TODO:
	// Call init program.
	// start("/usr/init");

	printf("shutdown\n");

	// TODO:
	// Shut the system down.
}
