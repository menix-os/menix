// Kernel entry point

#include <menix/common.h>
#include <menix/fs/ustar.h>
#include <menix/fs/vfs.h>
#include <menix/system/arch.h>
#include <menix/system/boot.h>
#include <menix/system/dst/core.h>
#include <menix/system/logger.h>
#include <menix/system/module.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/video/fbcon.h>
#include <menix/util/cmd.h>
#include <menix/util/log.h>

static BootInfo* info;

ATTR(noreturn) void kernel_init(BootInfo* boot_info)
{
	info = boot_info;

	// Initialize command line (without allocations), so we can control the allocator at boot time.
	cmd_early_init(boot_info->cmd);

	// Initialize basic IO.
	arch_early_init();

	// Initialize physical memory allocator.
	pm_init(boot_info->phys_base, boot_info->memory_map, boot_info->mm_num);

	// Initialize the kernel allocator.
	alloc_init();

	// Finalize virtual memory manager and drop reclaimable memory.
	vm_init(boot_info->kernel_phys, boot_info->memory_map, boot_info->mm_num);

	module_load_kernel_syms(info->kernel_file);

	// Initialize virtual file system.
	vfs_init();

	// If no early framebuffer has been set previously, do it now.
	if (info->fb && cmd_get_usize("fbcon", true))
	{
		fb_register(info->fb);
		fbcon_enable(true);
		fbcon_init();
	}

	// Say hello to the console!
	print_log("menix " MENIX_RELEASE " (" MENIX_ARCH ", " MENIX_VERSION ")\n");
	print_log("Command line: \"%s\"\n", info->cmd);

	// Load initrd(s).
	for (usize i = 0; i < info->file_num; i++)
		ustarfs_init(vfs_get_root(), info->files[i].address, info->files[i].size);

	// Now that we can allocate, copy over the command line so it doesn't get lost when we drop `.reclaim`.
	cmd_init();

	// Initialize firmware.
	print_log("boot: Initializing firmware.\n");
	dst_init(info);

	arch_init(boot_info);

	// Initialize all modules and subsystems.
	module_init();

	print_log("boot: Initialization complete, handing over to scheduler.\n");
	sch_init((VirtAddr)kernel_main);

	while (true)
		sch_arch_invoke();
}

ATTR(noreturn) void kernel_main()
{
	// Call init program.
	char* init_path = cmd_get_str("init", "/usr/sbin/init");
	// TODO: This variable shouldn't be neccessary if symlinks worked properly.
	char* init_name = cmd_get_str("init_name", "init");

	char* argv[] = {init_name, NULL};
	char* envp[] = {NULL};

	bool init_started = proc_create_elf(init_name, init_path, argv, envp, true);
	if (!init_started)
		print_error("Failed to run init binary! Try adding \"init=...\" to the command line.\n");

	// We're now in user space, stop printing stuff to the framebuffer.
	fbcon_enable(false);

	while (true)
		asm_pause();
}

ATTR(noreturn) void kernel_fini()
{
	// We're leaving user space, start printing stuff again.
	fbcon_enable(true);

	print_log("System is shutting down...\n");

	// If we're somehow still here, panic.
	panic();
}
