// Kernel entry point

#include <menix/boot.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/module.h>

void kernel_main(BootInfo* info)
{
	// Check if we got a framebuffer to draw on.
	if (info->fb_num == 0)
		kmesg("No framebuffer available! Forcing console to serial output.\n");
	else
	{
		// Draw the menix logo.
		// TODO: Get icon index from command line.
		for (usize i = 0; i < info->fb_num; i++)
		{
			fb_draw_bitmap(&info->fb[i], info->files[i].address, (info->fb[i].width >> 1) - (64 >> 1),
						   (info->fb[i].height >> 1) - (64 >> 1), 64, 64);
		}
	}

	// Say hello to the console.
	kmesg("menix v" CONFIG_version " (" CONFIG_arch ")\n");

	// Initialize all modules.
	module_init();

	// Init virtual file system.
	// vfs_init();
	// TODO: Get device to mount from command line.

	// TODO: Call init program.
	// exec("/usr/init");

	// Clean up all modules.
	module_fini();

	// TODO: Shut the system down.

	// Say goodbye.
	kmesg("shutdown\n");
}
