// Kernel entry point

#include <menix/common.h>
#include <menix/fs/ustar.h>
#include <menix/fs/vfs.h>
#include <menix/io/terminal.h>
#include <menix/system/arch.h>
#include <menix/system/fw.h>
#include <menix/system/module.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/util/log.h>

static BootInfo* boot_info;

ATTR(noreturn) void kernel_main()
{
	boot_info = fw_get_boot_info();

	// Initialize virtual file system.
	vfs_init();

	// Load initrd(s).
	for (usize i = 0; i < boot_info->file_num; i++)
		ustarfs_init(vfs_get_root(), boot_info->files[i].address, boot_info->files[i].size);

	// Register all terminal devices.
	terminal_init();

	// Initialize all modules and subsystems.
	module_init(boot_info);

	// Call init program.
	kassert(process_create_elf("init", ProcessState_Ready, arch_current_cpu()->thread->parent, "/sbin/init"),
			"Failed to run init binary!");

	// Should be unreachable.
	while (true)
	{
		asm_pause();
		sch_invoke();
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
