// Kernel entry point

#include <menix/common.h>
#include <menix/fs/ustar.h>
#include <menix/fs/vfs.h>
#include <menix/io/terminal.h>
#include <menix/system/arch.h>
#include <menix/system/boot.h>
#include <menix/system/dst/core.h>
#include <menix/system/module.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/util/cmd.h>
#include <menix/util/log.h>

void kernel_early_init(EarlyBootInfo* info)
{
	arch_early_init(info);

	// Initialize memory managers.
	pm_init(info->phys_base, info->memory_map, info->mm_num);
	vm_init(info->kernel_phys, info->memory_map, info->mm_num);
	alloc_init();

	// Initialize virtual file system.
	vfs_init();

	// Say hello to the console!
	kmesg("menix " CONFIG_release " (" CONFIG_arch ", " CONFIG_version ")\n");

	// Initialize command line.
	boot_log("Command line: \"%s\"\n", info->cmd);
	cmd_init(info->cmd);

	module_load_kernel_syms(info->kernel_file);
	boot_log("Kernel file loaded at: 0x%p\n", info->kernel_file);

	// Register all terminal devices.
	terminal_init();

	// Initialize firmware.
	boot_log("Initializing dynamic system tree.\n");
	dst_init(info);

	boot_log("Finished early initialization.\n");
}

ATTR(noreturn) void kernel_init(BootInfo* info)
{
	arch_init(info);

	// Load initrd(s).
	for (usize i = 0; i < info->file_num; i++)
		ustarfs_init(vfs_get_root(), info->files[i].address, info->files[i].size);

	// Initialize all modules and subsystems.
	module_init();

	boot_log("Initialization complete, handing over to scheduler.\n");
	sch_init();

	while (true)
		sch_invoke();
}

ATTR(noreturn) void kernel_main()
{
	// Call init program.
	char* init_path = cmd_get_str("init", "/usr/sbin/init");
	// TODO: This variable shouldn't be neccessary if symlinks worked properly.
	char* init_name = cmd_get_str("init_name", "init");

	char* argv[] = {init_name, NULL};
	char* envp[] = {NULL};

	bool init_started = proc_execve(init_name, init_path, argv, envp, true);
	kassert(init_started == true, "Failed to run init binary! Try adding \"init=...\" to the command line.");

	while (true)
		sch_invoke();
}
