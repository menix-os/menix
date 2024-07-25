// Limine bootloader entry point.

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

LIMINE_REQUEST struct limine_memmap_request memmap_request = {
	.id = LIMINE_MEMMAP_REQUEST,
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
	arch_early_init();
	boot_log("Initialized architecture\n");
	boot_log("Booting using Limine protocol\n");

	BootInfo info = {0};

	kassert(time_request.response, "Unable to get boot timestamp!\n") else
	{
		boot_log("Boot timestamp: %u\n", (u32)time_request.response->boot_time);
	}

#ifdef CONFIG_efi
	kassert(efi_st_request.response->address, "Unable to get EFI System Table!\n") else
	{
		info.efi_st = efi_st_request.response->address;
		boot_log("[EFI] System Table at 0x%p\n", info.efi_st);
		boot_log("[EFI] Number of table entries: %u\n", info.efi_st->NumberOfTableEntries);
	}
#endif

	kassert(memmap_request.response, "Unable to get memory map!\n") else
	{
		struct limine_memmap_response* const res = memmap_request.response;
		boot_log("Bootloader provided memory map at 0x%p\n", res);

		PhysMemory map[64];
		info.memory_map.num_blocks = res->entry_count;
		info.memory_map.blocks = map;

		for (usize i = 0; i < res->entry_count; i++)
		{
			map[i].address = res->entries[i]->base;
			map[i].length = res->entries[i]->length;

			const char* typ = "Unknown";
			switch (res->entries[i]->type)
			{
				case LIMINE_MEMMAP_USABLE:
					map[i].usage = PhysMemoryUsage_Free;
					typ = "Free";
					break;
				case LIMINE_MEMMAP_RESERVED:
					map[i].usage = PhysMemoryUsage_Reserved;
					typ = "Reserved";
					break;
				case LIMINE_MEMMAP_FRAMEBUFFER:
					map[i].usage = PhysMemoryUsage_Reserved;
					typ = "Framebuffer";
					break;
				default: map[i].usage = PhysMemoryUsage_Unknown; break;
			}

			boot_log("    Address = 0x%p Length = 0x%p Type = %s\n", map[i].address, map[i].length, typ);
		}
	}

	kassert(kernel_file_request.response, "Unable to get kernel file info!\n") else
	{
		struct limine_kernel_file_response* const res = kernel_file_request.response;
		boot_log("Kernel loaded at: 0x%p, Size = 0x%X\n", res->kernel_file->address, res->kernel_file->size);
		boot_log("Command line: \"%s\"\n", res->kernel_file->cmdline);
		info.cmd = res->kernel_file->cmdline;
	}

	kassert(framebuffer_request.response, "Unable to get a framebuffer!\n") else
	{
		boot_log("Got frame buffer:\n");
		FrameBuffer buffer;

		const struct limine_framebuffer* buf = framebuffer_request.response->framebuffers[0];
		buffer.base = buf->address;
		buffer.width = buf->width;
		buffer.height = buf->height;
		buffer.bpp = buf->bpp;
		buffer.pitch = buf->pitch;
		buffer.red_shift = buf->red_mask_shift;
		buffer.red_size = buf->red_mask_size;
		buffer.green_shift = buf->green_mask_shift;
		buffer.green_size = buf->green_mask_size;
		buffer.blue_shift = buf->blue_mask_shift;
		buffer.blue_size = buf->blue_mask_size;

		boot_log("    Address = 0x%p Width = %upx Height = %upx Pitch = %u\n", buffer.base, (u32)buffer.width,
				 (u32)buffer.height, (u32)buffer.pitch);

		// Fill framebuffer with sample data.
		fb_fill_pixels(&buffer, 0x00, 0x00, 0xFF);
		info.fb_num = 1;
		info.fb = &buffer;
	}

	arch_init(&info);

	boot_log("Handing control to main function\n");
	kernel_main(&info);
	boot_log("Got control back from main function\n");
	while (1)
		;
}
