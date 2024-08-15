// Kernel entry point

#include <menix/arch.h>
#include <menix/boot.h>
#include <menix/common.h>
#include <menix/fs/vfs.h>
#include <menix/io/terminal.h>
#include <menix/log.h>
#include <menix/module.h>

#ifdef CONFIG_acpi
#include <menix/drv/acpi/acpi.h>
#endif

void kernel_main(BootInfo* info)
{
	// TODO: Accept a cmdline option to keep the early terminal instead of reloading it.
	// Initialize console.
	// terminal_init();

	// Say hello to the console.
	kmesg("menix v" CONFIG_version " (" CONFIG_arch ")\n");

#ifdef CONFIG_acpi
	acpi_init(info->acpi_rsdp);
#endif

	// Init virtual file system.
	vfs_init();

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
