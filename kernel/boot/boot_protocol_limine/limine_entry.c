// Limine bootloader entry point.

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/pm.h>
#include <menix/system/arch.h>
#include <menix/system/boot.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/video/fb.h>
#include <menix/system/video/fb_default.h>
#include <menix/util/cmd.h>
#include <menix/util/log.h>

#include <string.h>

#define LIMINE_API_REVISION 1

#include "limine.h"

#define LIMINE_REQUEST(request, tag, rev) \
	ATTR(used, section(".requests")) static volatile struct limine_##request request = { \
		.id = tag, \
		.revision = rev, \
		.response = NULL, \
	}

ATTR(used, section(".requests_start_marker")) static volatile LIMINE_REQUESTS_START_MARKER;	   // Start requests
ATTR(used, section(".requests")) static volatile LIMINE_BASE_REVISION(3);

LIMINE_REQUEST(memmap_request, LIMINE_MEMMAP_REQUEST, 0);					 // Get memory map.
LIMINE_REQUEST(hhdm_request, LIMINE_HHDM_REQUEST, 0);						 // Directly map 32-bit physical space.
LIMINE_REQUEST(kernel_address_request, LIMINE_KERNEL_ADDRESS_REQUEST, 0);	 // Get the physical kernel address.
LIMINE_REQUEST(kernel_file_request, LIMINE_KERNEL_FILE_REQUEST, 0);			 // For debug symbols.
LIMINE_REQUEST(framebuffer_request, LIMINE_FRAMEBUFFER_REQUEST, 1);			 // Initial console frame buffer.
LIMINE_REQUEST(module_request, LIMINE_MODULE_REQUEST, 0);					 // Get all other modules.
LIMINE_REQUEST(rsdp_request, LIMINE_RSDP_REQUEST, 0);						 // Get ACPI RSDP table if enabled.
LIMINE_REQUEST(dtb_request, LIMINE_DTB_REQUEST, 0);							 // Get device tree blob if enabled.

ATTR(used, section(".requests_end_marker")) static volatile LIMINE_REQUESTS_END_MARKER;	   // End requests

static FrameBuffer early_fb;

void kernel_boot()
{
	BootInfo info = {0};

	kassert(memmap_request.response, "Unable to get memory map!");
	kassert(hhdm_request.response, "Unable to get HHDM response!");
	kassert(kernel_address_request.response, "Unable to get kernel address info!");
	kassert(kernel_file_request.response, "Unable to get kernel file info!");

	// Get the memory map.
	struct limine_memmap_response* const mm_res = memmap_request.response;
	PhysMemory map[mm_res->entry_count];
	for (usize i = 0; i < mm_res->entry_count; i++)
	{
		map[i].address = mm_res->entries[i]->base;
		map[i].length = mm_res->entries[i]->length;

		switch (mm_res->entries[i]->type)
		{
			case LIMINE_MEMMAP_USABLE: map[i].usage = PhysMemoryUsage_Free; break;
			case LIMINE_MEMMAP_KERNEL_AND_MODULES: map[i].usage = PhysMemoryUsage_Kernel; break;
			case LIMINE_MEMMAP_RESERVED:
			case LIMINE_MEMMAP_FRAMEBUFFER:
			case LIMINE_MEMMAP_ACPI_NVS: map[i].usage = PhysMemoryUsage_Reserved; break;
			case LIMINE_MEMMAP_ACPI_RECLAIMABLE:
			case LIMINE_MEMMAP_BOOTLOADER_RECLAIMABLE: map[i].usage = PhysMemoryUsage_Reclaimable; break;
			default: map[i].usage = PhysMemoryUsage_Unknown; break;
		}
	}

	// Get modules.
	if (module_request.response == NULL)
		print_log("limine: Unable to get modules, or none were provided!\n");
	else
	{
		const struct limine_module_response* module_res = module_request.response;
		BootFile* const files = info.files;
		print_log("limine: Got %zu module(s)\n", module_res->module_count);
		for (usize i = 0; i < module_res->module_count; i++)
		{
			files[i].address = module_res->modules[i]->address;
			files[i].size = module_res->modules[i]->size;
			files[i].path = module_res->modules[i]->path;
			print_log("limine: [%zu] Address = 0x%p, Size = 0x%zx, Path = \"%s\"\n", i, files[i].address, files[i].size,
					  files[i].path);
		}
		info.file_num = module_res->module_count;
	}

	// Get early framebuffer.
	if (framebuffer_request.response == NULL || framebuffer_request.response->framebuffer_count == 0)
		print_log("limine: Unable to get a framebuffer!\n");
	else
	{
		// Construct a simple framebuffer. This will get overridden by a driver loaded at a later stage.
		const struct limine_framebuffer* buf = framebuffer_request.response->framebuffers[0];
		info.fb = &early_fb;
		FrameBuffer* const buffer = info.fb;
		buffer->info.mmio_base = buf->address;
		buffer->mode.cpp = buf->bpp / 8;
		buffer->mode.width = buf->width;
		buffer->mode.v_width = buf->pitch / buffer->mode.cpp;
		buffer->mode.height = buf->height;
		buffer->mode.v_height = buf->height;
		buffer->mode.pitch = buf->pitch;
		buffer->funcs = FB_DEFAULT_FUNCS;
		print_log("limine: Early framebuffer: Address = 0x%p, Resolution = %ux%ux%hhu (Virtual = %ux%u)\n",
				  buffer->info.mmio_base, buffer->mode.width, buffer->mode.height, buffer->mode.cpp * 8,
				  buffer->mode.v_width, buffer->mode.v_height);
	}

	info.memory_map = map;
	info.mm_num = mm_res->entry_count;
	info.kernel_phys = (PhysAddr)kernel_address_request.response->physical_base;
	info.kernel_virt = (void*)kernel_address_request.response->virtual_base;
	info.kernel_file = (void*)kernel_file_request.response->kernel_file->address;
	info.phys_base = (void*)hhdm_request.response->offset;
	info.cmd = kernel_file_request.response->kernel_file->cmdline;

	if (rsdp_request.response)
		info.acpi_rsdp = rsdp_request.response->address;
	if (dtb_request.response)
		info.fdt_blob = dtb_request.response->dtb_ptr;

	kernel_init(&info);
}
