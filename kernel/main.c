// Kernel entry point

#include <menix/boot.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/module.h>

void kernel_main(BootInfo* info)
{
	// Say hello to the console.
	kmesg("menix v" MENIX_VERSION " (" MENIX_ARCH ")\n");

	// TODO: Parse the command line.

	// Draw the menix logo.
	// TODO: Get icon index from command line.
	usize icon_res = info->files[0].size / sizeof(u32);
	switch (icon_res)
	{
		case 32 * 32: icon_res = 32; break;
		case 64 * 64: icon_res = 64; break;
		case 128 * 128: icon_res = 128; break;
		case 256 * 256: icon_res = 256; break;
		case 512 * 512: icon_res = 512; break;
		case 1024 * 1024: icon_res = 1024; break;
		default: icon_res = 0; break;
	}
	fb_draw_bitmap(&info->fb[0], info->files[0].address, (info->fb[0].width >> 1) - (icon_res >> 1),
				   (info->fb[0].height >> 1) - (icon_res >> 1), icon_res, icon_res);

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
