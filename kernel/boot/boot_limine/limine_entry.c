// Limine bootloader entry point.

#include <menix/arch.h>
#include <menix/boot.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/util/self.h>

#include "limine.h"
#include "menix/memory/pm.h"

#define LIMINE_REQUEST(request, tag, rev) \
	ATTR(used, section(".requests")) static volatile struct limine_##request request = { \
		.id = tag, \
		.revision = rev, \
		.response = NULL, \
	}

ATTR(used, section(".requests_start_marker")) static volatile LIMINE_REQUESTS_START_MARKER;	   // Start requests
ATTR(used, section(".requests_end_marker")) static volatile LIMINE_REQUESTS_END_MARKER;		   // End requests
ATTR(used, section(".requests")) static volatile LIMINE_BASE_REVISION(2);

LIMINE_REQUEST(memmap_request, LIMINE_MEMMAP_REQUEST, 0);					 // Get memory map.
LIMINE_REQUEST(hhdm_request, LIMINE_HHDM_REQUEST, 0);						 // Directly map 32-bit physical space.
LIMINE_REQUEST(kernel_address_request, LIMINE_KERNEL_ADDRESS_REQUEST, 0);	 // Get the physical kernel address.
LIMINE_REQUEST(kernel_file_request, LIMINE_KERNEL_FILE_REQUEST, 0);			 // For debug symbols.
LIMINE_REQUEST(boot_time_request, LIMINE_BOOT_TIME_REQUEST, 0);				 // Get boot time stamp.
LIMINE_REQUEST(framebuffer_request, LIMINE_FRAMEBUFFER_REQUEST, 0);			 // Initial console frame buffer.
LIMINE_REQUEST(module_request, LIMINE_MODULE_REQUEST, 0);					 // Get all other modules, logo.
#ifdef CONFIG_acpi
LIMINE_REQUEST(rsdp_request, LIMINE_RSDP_REQUEST, 0);	 // Get ACPI RSDP table if enabled.
#endif
#ifdef CONFIG_open_firmware
LIMINE_REQUEST(dtb_request, LIMINE_DTB_REQUEST, 0);	   // Get device tree blob if enabled.
#endif

void kernel_boot()
{
	arch_early_init();
	boot_log("Initialized architecture\n");
	boot_log("Booting using Limine protocol\n");

	BootInfo info = {0};
	BootFile files[8];
	FrameBuffer buffer;

	// Remap the provided memory using our own.
	kassert(memmap_request.response, "Unable to get memory map!\n") else
	{
		struct limine_memmap_response* const res = memmap_request.response;
		boot_log("Bootloader provided memory map at 0x%p\n", res);
		boot_log("Free blocks:\n", res);

		PhysMemory map[res->entry_count];
		info.pm_num = res->entry_count;
		info.memory_map = map;

		usize total_free = 0;

		for (usize i = 0; i < res->entry_count; i++)
		{
			map[i].address = res->entries[i]->base;
			map[i].length = res->entries[i]->length;

			switch (res->entries[i]->type)
			{
				// Note: We treat Kernel maps like free memory as we do our own mapping.
				case LIMINE_MEMMAP_USABLE: map[i].usage = PhysMemoryUsage_Free; break;
				case LIMINE_MEMMAP_KERNEL_AND_MODULES: map[i].usage = PhysMemoryUsage_Kernel; break;
				case LIMINE_MEMMAP_RESERVED:
				case LIMINE_MEMMAP_ACPI_RECLAIMABLE:
				case LIMINE_MEMMAP_ACPI_NVS: map[i].usage = PhysMemoryUsage_Reserved; break;
				case LIMINE_MEMMAP_FRAMEBUFFER:
				case LIMINE_MEMMAP_BOOTLOADER_RECLAIMABLE: map[i].usage = PhysMemoryUsage_Bootloader; break;
				default: map[i].usage = PhysMemoryUsage_Unknown; break;
			}

			if (map[i].usage == PhysMemoryUsage_Free)
			{
				boot_log("    [%u] Address = 0x%p, Size = 0x%p\n", i, map[i].address, map[i].length);
				total_free += map[i].length;
			}
		}
		boot_log("    Total free = 0x%p (%u MiB)\n", total_free, total_free >> 20);

		// Make sure the first 4 GiB are identity mapped so we can write to "physical" memory.
		kassert(hhdm_request.response, "Unable to get HHDM response!\n") else
		{
			boot_log("HHDM offset: 0x%p\n", hhdm_request.response->offset);
		}
		kassert(kernel_address_request.response, "Unable to get kernel address info!\n") else
		{
			struct limine_kernel_address_response* const res = kernel_address_request.response;
			boot_log("Kernel loaded at: 0x%p (0x%p)\n", res->virtual_base, res->physical_base);
		}

		// Initialize virtual memory using the memory map we got.
		pm_init((void*)hhdm_request.response->offset, map, res->entry_count);
		vm_init((void*)hhdm_request.response->offset, (PhysAddr)kernel_address_request.response->physical_base, map,
				res->entry_count);
	}

	kassert(boot_time_request.response, "Unable to get boot timestamp!\n") else
	{
		boot_log("Boot timestamp: %u\n", (u32)boot_time_request.response->boot_time);
	}

#ifdef CONFIG_acpi
	kassert(rsdp_request.response, "Unable to get ACPI RSDP Table!\n") else
	{
		boot_log("ACPI System Table at 0x%p\n", rsdp_request.response->address);
		info.acpi_rsdp = rsdp_request.response->address;
	}
#endif

#ifdef CONFIG_open_firmware
	kassert(dtb_request.response, "Unable to get device tree!\n") else
	{
		boot_log("FDT blob at 0x%p\n", dtb_request.response->dtb_ptr);
		info.fdt_blob = dtb_request.response->dtb_ptr;
	}
#endif

	kassert(kernel_file_request.response, "Unable to get kernel file info!\n") else
	{
		struct limine_kernel_file_response* const res = kernel_file_request.response;
		boot_log("Kernel file loaded at: 0x%p, Size = 0x%X\n", res->kernel_file->address, res->kernel_file->size);

		self_set_kernel(res->kernel_file->address);

		boot_log("Command line: \"%s\"\n", res->kernel_file->cmdline);

		info.cmd = res->kernel_file->cmdline;
	}

	kassert(module_request.response, "Unable to get modules!\n") else
	{
		const struct limine_module_response* res = module_request.response;
		boot_log("Got files:\n");
		for (usize i = 0; i < res->module_count; i++)
		{
			files[i].address = res->modules[i]->address;
			files[i].size = res->modules[i]->size;
			files[i].path = res->modules[i]->path;
			boot_log("    Address = 0x%p, Size = 0x%p, Path = \"%s\"\n", files[i].address, files[i].size,
					 files[i].path);
		}
		info.file_num = res->module_count;
		info.files = files;
	}

	kassert(framebuffer_request.response, "Unable to get a framebuffer!\n") else
	{
		boot_log("Got frame buffer:\n");

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

		boot_log("    Address = 0x%p, Resolution = %ux%ux%u\n", buffer.base, buffer.width, buffer.height, buffer.bpp);

		info.fb_num = 1;
		info.fb = &buffer;
	}

	arch_init(&info);
	boot_log("Handing control to main function\n");
	kernel_main(&info);
	boot_log("Got control back from main function\n");
	arch_stop(&info);
}
