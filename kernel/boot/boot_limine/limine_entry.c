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

#include <string.h>

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
			case LIMINE_MEMMAP_USABLE: map[i].usage = PhysMemoryUsage_Free; break;
			case LIMINE_MEMMAP_KERNEL_AND_MODULES: map[i].usage = PhysMemoryUsage_Kernel; break;
			case LIMINE_MEMMAP_RESERVED:
			case LIMINE_MEMMAP_ACPI_NVS: map[i].usage = PhysMemoryUsage_Reserved; break;
			case LIMINE_MEMMAP_ACPI_RECLAIMABLE:
			case LIMINE_MEMMAP_FRAMEBUFFER:
			case LIMINE_MEMMAP_BOOTLOADER_RECLAIMABLE: map[i].usage = PhysMemoryUsage_Bootloader; break;
			default: map[i].usage = PhysMemoryUsage_Unknown; break;
		}
	}
	// Make sure the first 4 GiB are identity mapped so we can write to "physical" memory.
	kassert(hhdm_request.response, "Unable to get HHDM response!\n");
	kmesg("HHDM offset: 0x%p\n", hhdm_request.response->offset);
	kassert(kernel_address_request.response, "Unable to get kernel address info!\n");
	kmesg("Kernel loaded at: 0x%p (0x%p)\n", kernel_address_request.response->virtual_base,
		  kernel_address_request.response->physical_base);

	// Initialize virtual memory using the memory map we got.
	info.kernel_phys = (PhysAddr)kernel_address_request.response->physical_base;
	info.kernel_virt = (void*)kernel_address_request.response->virtual_base;
	info.phys_map = (void*)hhdm_request.response->offset;

#ifdef CONFIG_acpi
	// Get ACPI RSDP.
	kassert(rsdp_request.response, "Unable to get ACPI RSDP!\n");
	kmesg("ACPI RSDP at 0x%p\n", rsdp_request.response->address);
	info.acpi_rsdp = rsdp_request.response->address;
	kassert(memcmp(info.acpi_rsdp->signature, "RSD PTR", 7) == 0, "Invalid signature, expected \"RSD PTR\"!");
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
	kmesg("Got modules:\n");
	const struct limine_module_response* module_res = module_request.response;
	BootFile files[module_res->module_count];	 // TODO: Convert to kalloc'ed memory.
	for (usize i = 0; i < module_res->module_count; i++)
	{
		files[i].address = module_res->modules[i]->address;
		files[i].size = module_res->modules[i]->size;
		files[i].path = module_res->modules[i]->path;
		kmesg("    Address = 0x%p, Size = 0x%p, Path = \"%s\"\n", files[i].address, files[i].size, files[i].path);
	}
	info.file_num = module_res->module_count;
	info.files = files;

	// Get framebuffer.
	kassert(framebuffer_request.response, "Unable to get a framebuffer!\n");
	FrameBuffer buffers[framebuffer_request.response->framebuffer_count];
	info.fb_num = framebuffer_request.response->framebuffer_count;
	info.fb = buffers;
	for (usize i = 0; i < info.fb_num; i++)
	{
		const struct limine_framebuffer* buf = framebuffer_request.response->framebuffers[i];
		// Construct a simple framebuffer. This will get overridden by a driver loaded at a later stage.
		buffers[i].info.mmio_base = buf->address;
		buffers[i].mode.width = buf->width;
		buffers[i].mode.height = buf->height;
		buffers[i].mode.bpp = buf->bpp;

		fb_register(&buffers[i]);
	}

	arch_init(&info);
	kernel_main(&info);
}
