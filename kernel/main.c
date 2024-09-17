// Kernel entry point

#include <menix/common.h>
#include <menix/drv/module.h>
#include <menix/fs/ustar.h>
#include <menix/fs/vfs.h>
#include <menix/io/terminal.h>
#include <menix/system/arch.h>
#include <menix/thread/scheduler.h>
#include <menix/util/log.h>

static BootInfo* boot_info;

ATTR(noreturn) void kernel_main(BootInfo* info)
{
	boot_info = info;

	vfs_init();

	// Load initrd(s).
	for (usize i = 0; i < info->file_num; i++)
		ustarfs_init(vfs_get_root(), info->files[i].address, info->files[i].size);

	// Register all terminal devices.
	terminal_init();

	// Initialize all modules and subsystems.
	module_init(boot_info);

	// Call init program.
	char* argv[] = {"init", NULL};
	process_execve("/sbin/init", argv, NULL);

	kmesg("[WARNING]\tinit has terminated!\n");

	// Should be unreachable.
	while (true)
	{
		asm_pause();
		scheduler_invoke();
	}
}

void kernel_shutdown(i32 reason)
{
	// Say goodbye.
	kmesg("System is shutting down...\n");

	// Clean up all modules and subsystems.
	module_fini();

	// Shut the system down safely.
	arch_shutdown(boot_info);

	// If we're still here, something went wrong. In that case, just try to stop.
	arch_stop(boot_info);
}
