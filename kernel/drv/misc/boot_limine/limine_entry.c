//? Limine bootloader entry point.

#include <menix/arch.h>
#include <menix/boot.h>
#include <menix/common.h>
#include <menix/format.h>
#include <menix/log.h>

#include "limine.h"

#define LIMINE_REQUEST ATTR(section(".requests")) ATTR(used) static volatile

// Start requests
ATTR(used) ATTR(section(".requests_start_marker")) static volatile LIMINE_REQUESTS_START_MARKER;

LIMINE_REQUEST LIMINE_BASE_REVISION(2);

LIMINE_REQUEST struct limine_framebuffer_request framebuffer_request = {
	.id = LIMINE_FRAMEBUFFER_REQUEST,
	.revision = 0,
};

LIMINE_REQUEST struct limine_kernel_file_request kernel_file_request = {
	.id = LIMINE_KERNEL_FILE_REQUEST,
	.revision = 0,
};

#ifdef CONFIG_efi
LIMINE_REQUEST struct limine_efi_system_table_request efi_st_request = {
	.id = LIMINE_EFI_SYSTEM_TABLE_REQUEST,
	.revision = 0,
};
#endif

LIMINE_REQUEST struct limine_boot_time_request time_request = {
	.id = LIMINE_BOOT_TIME_REQUEST,
	.revision = 0,
};

// End requests
ATTR(used) ATTR(section(".requests_end_marker")) static volatile LIMINE_REQUESTS_END_MARKER;

void kernel_boot()
{
	arch_init();
	boot_log("Booting using Limine protocol\n");
	boot_log("Initialized architecture\n");

	BootInfo info = { 0 };

	kassert(time_request.response, "Unable to get boot timestamp!\n") else
	{
		boot_log("Boot timestamp: %u\n", (uint32_t)time_request.response->boot_time);
	}

#ifdef CONFIG_efi
	kassert(efi_st_request.response->address, "Unable to get EFI System Table!\n") else
	{
		boot_log("EFI System Table at %#p\n", efi_st_request.response->address);
		info.efi_st = efi_st_request.response->address;
	}
#endif

	kassert(kernel_file_request.response, "Unable to get kernel file info!\n") else
	{
		boot_log("Command line: \"%s\"\n", kernel_file_request.response->kernel_file->cmdline);
		info.cmd = kernel_file_request.response->kernel_file->cmdline;
	}

	kassert(framebuffer_request.response, "Unable to get a framebuffer!\n") else
	{
		boot_log("Got frame buffers:\n");
		FrameBuffer buffers[FB_MAX];
		size_t		total_fbs = 0;
		for (uint64_t i = 0; i < framebuffer_request.response->framebuffer_count; i++)
		{
			const struct limine_framebuffer* buf = framebuffer_request.response->framebuffers[i];
			buffers[i].base = buf->address;
			buffers[i].width = buf->width;
			buffers[i].height = buf->height;
			buffers[i].bpp = buf->bpp;
			buffers[i].pitch = buf->pitch;
			buffers[i].red_shift = buf->red_mask_shift;
			buffers[i].red_size = buf->red_mask_size;
			buffers[i].green_shift = buf->green_mask_shift;
			buffers[i].green_size = buf->green_mask_size;
			buffers[i].blue_shift = buf->blue_mask_shift;
			buffers[i].blue_size = buf->blue_mask_size;

			boot_log("\t[%u] Address: %#p\tWidth: %upx\tHeight: %upx\tPitch: %u\n", i, buffers[i].base,
					 (uint32_t)buffers[i].width, (uint32_t)buffers[i].height, (uint32_t)buffers[i].pitch);

			// Fill framebuffer with sample data.
			fb_fill_pixels(&buffers[i], 0x20, 0x7F, 0x70);

			if (i >= FB_MAX - 1)
			{
				total_fbs = i;
				break;
			}
		};
		info.fb_num = total_fbs;
		info.fb = buffers;
	}

	boot_log("Handing control to main function\n");
	kernel_main(&info);
	boot_log("Got control back from main function\n");

	while (1)
		;
}
