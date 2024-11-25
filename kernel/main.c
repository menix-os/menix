// Kernel entry point

#include <menix/common.h>
#include <menix/fs/ustar.h>
#include <menix/fs/vfs.h>
#include <menix/io/terminal.h>
#include <menix/system/arch.h>
#include <menix/system/boot.h>
#include <menix/system/fw.h>
#include <menix/system/module.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/util/cmd.h>
#include <menix/util/log.h>

void kernel_early_init(BootInfo* info)
{
	arch_early_init(info);

	// Say hello to the console!
	kmesg("menix " CONFIG_release " (" CONFIG_arch ", " CONFIG_version ")\n");

	// Initialize memory managers.
	pm_init(info->phys_base, info->memory_map, info->mm_num);
	vm_init(info->kernel_phys, info->memory_map, info->mm_num);
	alloc_init();

	// Initialize command line.
	cmd_init(info);

	// Initialize virtual file system.
	vfs_init();

	// Register all terminal devices.
	terminal_init();
}

void kernel_init(BootInfo* info)
{
	arch_init(info);

	// Load initrd(s).
	for (usize i = 0; i < info->file_num; i++)
		ustarfs_init(vfs_get_root(), info->files[i].address, info->files[i].size);

	// Initialize all modules and subsystems.
	module_init(info);

	boot_log("Initialization complete, handing over to scheduler.\n");
	sch_init();
}

ATTR(noreturn) void kernel_main()
{
	// Call init program.
	char* init_path = cmd_get_str("init", "/usr/sbin/init");
	char* init_name = cmd_get_str("init_name", "init");

	char* argv[] = {init_name, NULL};
	char* envp[] = {NULL};

	bool init_started = proc_execve(init_name, init_path, argv, envp, true);
	kassert(init_started == true, "Failed to run init binary! Try adding \"init=...\" to the command line.");

	while (true)
		sch_invoke();
}
