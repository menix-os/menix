// Kernel entry point

#include <menix/boot.h>
#include <menix/common.h>
#include <menix/io/terminal.h>
#include <menix/log.h>
#include <menix/module.h>

void kernel_main(BootInfo* info)
{
	// Say hello to the console.
	kmesg("menix v" CONFIG_version " (" CONFIG_arch ")\n");

	// Initialize all modules.
	module_init();

	// Init virtual file system.
	// vfs_init();
	// TODO: Get device to mount from command line.

	// TODO: Call init program.
	// exec("/usr/init");

	// Clean up all modules.
	module_fini();

	// TODO: Shut the system down.

	// Say goodbye.
	kmesg("shutdown\n");
}
