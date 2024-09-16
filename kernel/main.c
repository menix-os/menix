// Kernel entry point

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/fs/ustar.h>
#include <menix/fs/vfs.h>
#include <menix/io/terminal.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/sys/syscall_list.h>
#include <menix/thread/process.h>

static BootInfo* boot_info;

ATTR(noreturn) void kernel_main(BootInfo* info)
{
	boot_info = info;

	// TODO: Move this to scheduler.
	arch_current_cpu()->thread = kzalloc(sizeof(Thread));
	arch_current_cpu()->thread->parent = kzalloc(sizeof(Process));
	arch_current_cpu()->thread->parent->map_base = CONFIG_vm_map_base;

	vfs_init();

	// Load initrd(s).
	for (usize i = 0; i < info->file_num; i++)
		ustarfs_init(vfs_get_root(), info->files[i].address, info->files[i].size);

	// Register all terminal devices.
	terminal_init();

	// Initialize all modules and subsystems.
	module_init(boot_info);

	// Say hello to the console.
	struct utsname uname;
	syscall_uname((usize)&uname, 0, 0, 0, 0, 0);
	kmesg("%s %s [%s] %s\n", uname.sysname, uname.release, uname.version, uname.machine);

	// Call init program.
	char* argv[] = {"init", NULL};
	process_execve("/sbin/init", argv, NULL);

	while (true)
	{
		// sched_invoke();
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
