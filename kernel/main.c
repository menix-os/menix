// Kernel entry point

#include <menix/boot.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/module.h>

void kernel_main(BootInfo * info)
{
	// Say hello to the console.
	kmesg("menix v" MENIX_VERSION " (" MENIX_ARCH ")\n");

	// Initialize all modules.
	module_init();

	// Init virtual file system.
	// vfs_init();

	// TODO: Call init program.
	// exec("/usr/init");

	// Clean up all modules.
	module_fini();
	
	// TODO: Shut the system down.

	// Say goodbye.
	kmesg("shutdown\n");
}
