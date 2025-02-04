// Kernel entry point

#include <menix/common.h>
#include <menix/io/terminal.h>
#include <menix/system/arch.h>
#include <menix/system/boot.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
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

	// Initialize memory managers.
	pm_init(boot_info->phys_base, boot_info->memory_map, boot_info->mm_num);
	alloc_init();

	// Finalize virtual memory manager and drop reclaimable memory.
	vm_init(boot_info->kernel_phys, boot_info->memory_map, boot_info->mm_num);

	terminal_init();

	// Now that we can allocate, copy over the command line so it doesn't get lost when we drop `.reclaim`.
	cmd_init();

	arch_init(boot_info);

	print_log("boot: Initialization complete, handing over to scheduler.\n");
	sch_init((VirtAddr)kernel_main);

	while (true)
		sch_arch_invoke();
}

ATTR(noreturn) void kernel_main()
{
	// If no early framebuffer has been set previously, do it now.
	fb_register(info->fb);

	// Say hello to the console!
	print_log("menix " MENIX_RELEASE " (" MENIX_ARCH ", " MENIX_VERSION ")\n");
	print_log("boot: Command line: \"%s\"\n", info->cmd);

	char* argv[] = {"init", NULL};
	char* envp[] = {NULL};

	for (usize i = 0; i < info->file_num; i++)
	{
		bool init_started = proc_create_elf("init", info->files[i].address, info->files[i].size, argv, envp);
		kassert(init_started == true, "Failed to run startup binary \"%s\"", info->files[i].path);
	}

	while (true)
		asm_pause();
}
