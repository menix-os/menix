#include <menix/acpi.h>
#include <menix/cmdline.h>
#include <menix/init.h>
#include <menix/mem.h>
#include <menix/log.h>
#include <menix/util.h>
#include <string.h>

#include "limine.h"

__initdata_named(req_0) static volatile LIMINE_REQUESTS_START_MARKER;
__initdata_named(req_1) static volatile LIMINE_BASE_REVISION(3);
__initdata_named(req_2) static volatile LIMINE_REQUESTS_END_MARKER;

#define LIMINE_REQUEST(request, tag, rev) \
	__initdata_named(req_1) static volatile struct limine_##request request = { \
		.id = tag, \
		.revision = rev, \
		.response = nullptr, \
	}

LIMINE_REQUEST(memmap_request, LIMINE_MEMMAP_REQUEST, 0);
LIMINE_REQUEST(hhdm_request, LIMINE_HHDM_REQUEST, 0);
LIMINE_REQUEST(executable_address_request, LIMINE_EXECUTABLE_ADDRESS_REQUEST, 0);
LIMINE_REQUEST(executable_file_request, LIMINE_EXECUTABLE_FILE_REQUEST, 0);
LIMINE_REQUEST(framebuffer_request, LIMINE_FRAMEBUFFER_REQUEST, 1);
LIMINE_REQUEST(module_request, LIMINE_MODULE_REQUEST, 0);
LIMINE_REQUEST(rsdp_request, LIMINE_RSDP_REQUEST, 0);
LIMINE_REQUEST(dtb_request, LIMINE_DTB_REQUEST, 0);

__init void _start() {
	kassert(memmap_request.response, "Unable to get memory map!");
	kassert(hhdm_request.response, "Unable to get HHDM response!");
	kassert(executable_address_request.response, "Unable to get kernel address info!");
	kassert(executable_file_request.response, "Unable to get kernel file info!");

	// Get the memory map.
	struct limine_memmap_response* const mm_res = memmap_request.response;
	mem_map_size = min(mm_res->entry_count, array_size(mem_map));
	if (mm_res->entry_count > mem_map_size)
		kwarn("Truncating %zu memory map entries!", mm_res->entry_count - mem_map_size);

	for (usize i = 0; i < mem_map_size; i++) {
		mem_map[i].address = mm_res->entries[i]->base;
		mem_map[i].length = mm_res->entries[i]->length;

		switch (mm_res->entries[i]->type) {
		case LIMINE_MEMMAP_USABLE:
			mem_map[i].usage = PHYS_USABLE;
			break;
		case LIMINE_MEMMAP_ACPI_RECLAIMABLE:
		case LIMINE_MEMMAP_BOOTLOADER_RECLAIMABLE:
			mem_map[i].usage = PHYS_RECLAIMABLE;
			break;
		default:
			mem_map[i].usage = PHYS_RESERVED;
			break;
		}
	}

	// Get modules.
	if (module_request.response == nullptr)
		kerror("limine: Unable to get modules, or none were provided!\n");
	else {
		const struct limine_module_response* module_res = module_request.response;
		klog("limine: Got %zu module(s)\n", module_res->module_count);
		boot_files_count = min(module_res->module_count, array_size(boot_files));
		for (usize i = 0; i < boot_files_count; i++) {
			boot_files[i].address = module_res->modules[i]->address;
			boot_files[i].length = module_res->modules[i]->size;
			boot_files[i].path = module_res->modules[i]->path;
			kmsg(
				"limine: [%zu] Address = 0x%p, Size = 0x%zx, Path = \"%s\"\n", i, boot_files[i].address,
				boot_files[i].length, boot_files[i].path
			);
		}
	}

	mem_kernel_phys_base = (phys_t)executable_address_request.response->physical_base;
	mem_kernel_virt_base = (virt_t)executable_address_request.response->virtual_base;
	mem_hhdm_base = (virt_t)hhdm_request.response->offset;

	const char* cmdline = executable_file_request.response->executable_file->string;
	memcpy(cmdline_buffer, cmdline, min(strlen(cmdline), array_size(cmdline_buffer)));

	if (rsdp_request.response)
		acpi_rsdp_address = (phys_t)rsdp_request.response->address;

	kmain();
}
