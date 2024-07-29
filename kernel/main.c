// Kernel entry point

#include <menix/boot.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/module.h>

void kernel_main(BootInfo* info)
{
	// Say hello to the console.
	kmesg("menix v" CONFIG_version " (" CONFIG_arch ")\n");

	// TODO: Parse the command line.

	// Check if we got a framebuffer to draw on.
	kassert(info->fb_num >= 1, "No framebuffer available! Forcing console to serial output.\n") else
	{
		// Draw the menix logo.
		// TODO: Get icon index from command line.
		kassert(info->files[0].size == 64 * 64 * sizeof(u32), "Icon must be 64x64 pixels in BGRA32 format.") else
		{
			fb_draw_bitmap(&info->fb[0], info->files[0].address, (info->fb[0].width >> 1) - (64 >> 1),
						   (info->fb[0].height >> 1) - (64 >> 1), 64, 64);
		}
	}

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
