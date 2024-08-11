// Kernel entry point

#include <menix/arch.h>
#include <menix/boot.h>
#include <menix/common.h>
#include <menix/io/terminal.h>
#include <menix/log.h>
#include <menix/module.h>

void kernel_main(BootInfo* info)
{
	// Initialize console.
	terminal_init();

	// Say hello to the console.
	kmesg("menix v" CONFIG_version " (" CONFIG_arch ")\n");

	// Init virtual file system.
	// vfs_init();

	// Initialize all modules.
	module_init();

	// TODO: Call init program.
	// exec("/usr/init");

	// Clean up all modules.
	module_fini();

	kmesg("shutdown\n");	// Say goodbye.
	arch_shutdown(info);	// Shut the system down safely.
	arch_stop(info);		// If we're still here, something went wrong. In that case, just try to stop.
}
