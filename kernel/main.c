// Kernel entry point

#include <menix/arch.h>
#include <menix/boot.h>
#include <menix/common.h>
#include <menix/fs/vfs.h>
#include <menix/io/terminal.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/util/list.h>

void kernel_main(BootInfo* info)
{
	// TODO: Accept a cmdline option to keep the early terminal instead of reloading it.
	// Initialize console.
	// terminal_init();

	// Say hello to the console.
	kmesg("menix v" CONFIG_version " (" CONFIG_arch ")\n");

	// Init virtual file system.
	vfs_init();

	// Initialize all modules.
	module_init(info);

	// TODO: Call init program.
	// exec("/usr/init");

	// Clean up all modules.
	module_fini();

	kmesg("shutdown\n");	// Say goodbye.
	arch_shutdown(info);	// Shut the system down safely.
	arch_stop(info);		// If we're still here, something went wrong. In that case, just try to stop.
}
