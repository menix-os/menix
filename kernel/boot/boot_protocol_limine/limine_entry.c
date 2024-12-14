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

#include "limine.h"

#define LIMINE_REQUEST(request, tag, rev) \
	ATTR(used, section(".requests")) static volatile struct limine_##request request = { \
		.id = tag, \
		.revision = rev, \
		.response = NULL, \
	}

ATTR(used, section(".requests_start_marker")) static volatile LIMINE_REQUESTS_START_MARKER;	   // Start requests
ATTR(used, section(".requests")) static volatile LIMINE_BASE_REVISION(2);

LIMINE_REQUEST(memmap_request, LIMINE_MEMMAP_REQUEST, 0);					 // Get memory map.
LIMINE_REQUEST(hhdm_request, LIMINE_HHDM_REQUEST, 0);						 // Directly map 32-bit physical space.
LIMINE_REQUEST(kernel_address_request, LIMINE_KERNEL_ADDRESS_REQUEST, 0);	 // Get the physical kernel address.
LIMINE_REQUEST(kernel_file_request, LIMINE_KERNEL_FILE_REQUEST, 0);			 // For debug symbols.
LIMINE_REQUEST(framebuffer_request, LIMINE_FRAMEBUFFER_REQUEST, 1);			 // Initial console frame buffer.
LIMINE_REQUEST(module_request, LIMINE_MODULE_REQUEST, 0);					 // Get all other modules.
LIMINE_REQUEST(smp_request, LIMINE_SMP_REQUEST, 0);							 // Get SMP information.

#ifdef CONFIG_acpi
LIMINE_REQUEST(rsdp_request, LIMINE_RSDP_REQUEST, 0);	 // Get ACPI RSDP table if enabled.
#endif

#ifdef CONFIG_open_firmware
LIMINE_REQUEST(dtb_request, LIMINE_DTB_REQUEST, 0);	   // Get device tree blob if enabled.
#endif

ATTR(used, section(".requests_end_marker")) static volatile LIMINE_REQUESTS_END_MARKER;	   // End requests

static BootInfo full_info = {0};

static void limine_init_cpu(struct limine_smp_info* smp_info)
{
	arch_init_cpu(&full_info, (Cpu*)smp_info->extra_argument, &full_info.cpus[full_info.boot_cpu]);
}

void kernel_boot()
{
	kassert(memmap_request.response, "Unable to get memory map!");
	kassert(hhdm_request.response, "Unable to get HHDM response!");
	kassert(kernel_address_request.response, "Unable to get kernel address info!");
	kassert(kernel_file_request.response, "Unable to get kernel file info!");

	EarlyBootInfo early_info = {0};

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

	early_info = (EarlyBootInfo) {
		.memory_map = map,
		.mm_num = mm_res->entry_count,
		.kernel_phys = (PhysAddr)kernel_address_request.response->physical_base,
		.kernel_virt = (void*)kernel_address_request.response->virtual_base,
		.kernel_file = (void*)kernel_file_request.response->kernel_file->address,
		.phys_base = (void*)hhdm_request.response->offset,
		.cmd = kernel_file_request.response->kernel_file->cmdline,
#ifdef CONFIG_acpi
		.acpi_rsdp = rsdp_request.response->address,
#endif
#ifdef CONFIG_open_firmware
		.fdt_blob = dtb_request.response->dtb_ptr,
#endif
	};

	kernel_early_init(&early_info);

	// Get modules.
	if (module_request.response == NULL)
		print_log("boot: Unable to get modules, or none were provided!\n");
	else
	{
		print_log("boot: Got modules:\n");
		const struct limine_module_response* module_res = module_request.response;
		BootFile* files = kmalloc(sizeof(BootFile) * module_res->module_count);
		for (usize i = 0; i < module_res->module_count; i++)
		{
			files[i].address = module_res->modules[i]->address;
			files[i].size = module_res->modules[i]->size;
			files[i].path = module_res->modules[i]->path;
			print_log("boot: \t[%i] Address = 0x%p, Size = 0x%zx, Path = \"%s\"\n", i, files[i].address, files[i].size,
					  files[i].path);
		}
		full_info.file_num = module_res->module_count;
		full_info.files = files;
	}

	// Get early framebuffer.
	if (framebuffer_request.response == NULL || framebuffer_request.response->framebuffer_count == 0)
		print_log("boot: Unable to get a framebuffer!\n");
	else
	{
		// Construct a simple framebuffer. This will get overridden by a driver loaded at a later stage.
		const struct limine_framebuffer* buf = framebuffer_request.response->framebuffers[0];

		FrameBuffer* buffer = kzalloc(sizeof(FrameBuffer));
		buffer->info.mmio_base = buf->address;
		buffer->mode.cpp = buf->bpp / 8;
		buffer->mode.width = buf->width;
		buffer->mode.v_width = buf->pitch / buffer->mode.cpp;
		buffer->mode.height = buf->height;
		buffer->mode.v_height = buf->height;
		buffer->mode.pitch = buf->pitch;
		buffer->funcs = FB_DEFAULT_FUNCS;

		// If no early framebuffer has been set previously, do it now.
		if (fb_get_active() == NULL)
			fb_register(buffer);
		print_log("boot: Early framebuffer: Address = 0x%p, Resolution = %ux%ux%hhu (Virtual = %ux%u)\n",
				  buffer->info.mmio_base, buffer->mode.width, buffer->mode.height, buffer->mode.cpp * 8,
				  buffer->mode.v_width, buffer->mode.v_height);
	}

	// TODO: Instead of relying on the bootloader to do SMP, do this ourselves.
	// Get SMP info
	kassert(smp_request.response, "Unable to get kernel SMP info!");
	const struct limine_smp_response* smp_res = smp_request.response;
	usize smp_cmdline = cmd_get_usize("smp", smp_res->cpu_count);
	// Only initialize a given amount of cores, or if none/invalid the maximum cpu count.
	full_info.cpu_num = (smp_cmdline > smp_res->cpu_count || smp_cmdline == 0) ? smp_res->cpu_count : smp_cmdline;
	print_log("boot: Initializing %zu cores.\n", full_info.cpu_num);

	// Mark the boot CPU ID.
#ifdef CONFIG_arch_x86_64
	full_info.boot_cpu = smp_res->bsp_lapic_id;
#elif defined(CONFIG_arch_riscv64)
	info.boot_cpu = smp_res->bsp_hartid;
#endif

	// Allocate CPU info.
	full_info.cpus = kzalloc(sizeof(Cpu) * full_info.cpu_num);

	// Start all cores.
	for (usize i = 0; i < full_info.cpu_num; i++)
	{
		struct limine_smp_info* smp_cpu = smp_res->cpus[i];
		smp_cpu->extra_argument = (usize)&full_info.cpus[i];
		Cpu* cpu = &full_info.cpus[i];
		cpu->id = i;
#ifdef CONFIG_arch_x86_64
		cpu->lapic_id = smp_cpu->lapic_id;
		if (cpu->lapic_id != full_info.boot_cpu)
#elif defined(CONFIG_arch_riscv64)
		cpu->hart_id = smp_cpu->hartid;
		if (cpu->hart_id != info.boot_cpu)
#endif
			smp_cpu->goto_address = limine_init_cpu;
		else
			limine_init_cpu(smp_cpu);
	}
	while (full_info.cpu_active != full_info.cpu_num)
	{
		__sync_synchronize();
		asm_pause();
	}

	kernel_init(&full_info);
}
