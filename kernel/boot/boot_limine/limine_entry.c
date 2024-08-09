// Limine bootloader entry point.

#include <menix/arch.h>
#include <menix/boot.h>
#include <menix/common.h>
#include <menix/io/terminal.h>
#include <menix/log.h>
#include <menix/memory/alloc.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/util/self.h>

#include "limine.h"

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

	BootInfo info = {0};

	// Get framebuffer.
	kassert(framebuffer_request.response, "Unable to get a framebuffer!\n");
	kmesg("Got frame buffer:\n");
	// TODO: Convert to kalloc'ed memory.
	FrameBuffer buffers[framebuffer_request.response->framebuffer_count];
	info.fb_num = framebuffer_request.response->framebuffer_count;
	info.fb = buffers;
	for (usize i = 0; i < info.fb_num; i++)
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
	}
	// Check if we got a framebuffer to draw on and initialize console if we do.
	if (info.fb && info.fb_num > 0)
		terminal_init(info.fb);

	// Get the memory map.
	kassert(memmap_request.response, "Unable to get memory map!\n");
	struct limine_memmap_response* const res = memmap_request.response;
	kmesg("Bootloader provided memory map at 0x%p\n", res);

	PhysMemory map[res->entry_count];
	info.mm_num = res->entry_count;
	info.memory_map = map;

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
	}
	// Make sure the first 4 GiB are identity mapped so we can write to "physical" memory.
	kassert(hhdm_request.response, "Unable to get HHDM response!\n");
	kmesg("HHDM offset: 0x%p\n", hhdm_request.response->offset);
	kassert(kernel_address_request.response, "Unable to get kernel address info!\n")
		kmesg("Kernel loaded at: 0x%p (0x%p)\n", kernel_address_request.response->virtual_base,
			  kernel_address_request.response->physical_base);

	// Initialize virtual memory using the memory map we got.
	info.kernel_phys = (PhysAddr)kernel_address_request.response->physical_base;
	info.kernel_virt = (void*)kernel_address_request.response->virtual_base;
	info.phys_map = (void*)hhdm_request.response->offset;

	// Get boot timestamp.
	kassert(boot_time_request.response, "Unable to get boot timestamp!\n");
	kmesg("Boot timestamp: %u\n", (u32)boot_time_request.response->boot_time);

#ifdef CONFIG_acpi
	// Get ACPI RSDP table.
	kassert(rsdp_request.response, "Unable to get ACPI RSDP table!\n");
	kmesg("ACPI System Table at 0x%p\n", rsdp_request.response->address);
	info.acpi_rsdp = rsdp_request.response->address;
#endif

#ifdef CONFIG_open_firmware
	kassert(dtb_request.response, "Unable to get device tree!\n");
	kmesg("FDT blob at 0x%p\n", dtb_request.response->dtb_ptr);
	info.fdt_blob = dtb_request.response->dtb_ptr;
#endif

	// Get kernel file.
	kassert(kernel_file_request.response, "Unable to get kernel file info!\n");
	struct limine_kernel_file_response* const kernel_res = kernel_file_request.response;
	kmesg("Kernel file loaded at: 0x%p, Size = 0x%X\n", kernel_res->kernel_file->address,
		  kernel_res->kernel_file->size);

	self_set_kernel(kernel_res->kernel_file->address);

	// Get command line.
	kmesg("Command line: \"%s\"\n", kernel_res->kernel_file->cmdline);
	info.cmd = kernel_res->kernel_file->cmdline;

	// Get modules.
	kassert(module_request.response, "Unable to get modules!\n");
	kmesg("Got files:\n");
	const struct limine_module_response* module_res = module_request.response;
	// TODO: Convert to kalloc'ed memory.
	BootFile files[module_res->module_count];

	for (usize i = 0; i < module_res->module_count; i++)
	{
		files[i].address = module_res->modules[i]->address;
		files[i].size = module_res->modules[i]->size;
		files[i].path = module_res->modules[i]->path;
		kmesg("    Address = 0x%p, Size = 0x%p, Path = \"%s\"\n", files[i].address, files[i].size, files[i].path);
	}
	info.file_num = module_res->module_count;
	info.files = files;

	arch_init(&info);
	kernel_main(&info);
	arch_stop(&info);
}
