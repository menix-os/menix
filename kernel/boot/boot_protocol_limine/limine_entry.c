// Limine bootloader entry point.

#include <menix/arch.h>
#include <menix/boot.h>
#include <menix/common.h>
#include <menix/io/terminal.h>
#include <menix/log.h>
#include <menix/memory/alloc.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/thread/elf.h>
#include <menix/video/fb.h>
#include <menix/video/fb_default.h>

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
LIMINE_REQUEST(framebuffer_request, LIMINE_FRAMEBUFFER_REQUEST, 1);			 // Initial console frame buffer.
LIMINE_REQUEST(module_request, LIMINE_MODULE_REQUEST, 0);					 // Get all other modules, logo.
#ifdef CONFIG_smp
LIMINE_REQUEST(smp_request, LIMINE_SMP_REQUEST, 0);	   // Get SMP information.
#endif
#ifdef CONFIG_acpi
LIMINE_REQUEST(rsdp_request, LIMINE_RSDP_REQUEST, 0);	 // Get ACPI RSDP table if enabled.
#endif
#ifdef CONFIG_open_firmware
LIMINE_REQUEST(dtb_request, LIMINE_DTB_REQUEST, 0);	   // Get device tree blob if enabled.
#endif

static BootInfo info = {0};

static void limine_init_cpu(struct limine_smp_info* smp_info)
{
	arch_init_cpu((Cpu*)smp_info->extra_argument, &info.cpus[info.boot_cpu]);
}

void kernel_boot()
{
	arch_early_init(&info);

	// Get the memory map.
	kassert(memmap_request.response, "Unable to get memory map!");
	struct limine_memmap_response* const mm_res = memmap_request.response;
	boot_kmesg("Bootloader provided memory map at 0x%p\n", mm_res);

	PhysMemory map[mm_res->entry_count];
	info.mm_num = mm_res->entry_count;
	info.memory_map = map;

	for (usize i = 0; i < mm_res->entry_count; i++)
	{
		map[i].address = mm_res->entries[i]->base;
		map[i].length = mm_res->entries[i]->length;

		switch (mm_res->entries[i]->type)
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
	kassert(hhdm_request.response, "Unable to get HHDM response!");
	kassert(kernel_address_request.response, "Unable to get kernel address info!");

	// Initialize virtual memory using the memory map we got.
	info.kernel_phys = (PhysAddr)kernel_address_request.response->physical_base;
	info.kernel_virt = (void*)kernel_address_request.response->virtual_base;
	info.phys_map = (void*)hhdm_request.response->offset;

	// Initialize physical and virtual memory managers.
	pm_init(info.phys_map, info.memory_map, info.mm_num);
	vm_init(info.phys_map, info.kernel_phys, info.memory_map, info.mm_num);
	// Initialize memory allocator.
	alloc_init();
	// Get early framebuffer.
	FrameBuffer buffer = {0};

	if (framebuffer_request.response == NULL || framebuffer_request.response->framebuffer_count == 0)
		boot_kmesg("Unable to get a framebuffer!\n");
	else
	{
		// Construct a simple framebuffer. This will get overridden by a driver loaded at a later stage.
		const struct limine_framebuffer* buf = framebuffer_request.response->framebuffers[0];
		buffer.info.mmio_base = buf->address;
		buffer.mode.cpp = buf->bpp / 8;
		buffer.mode.width = buf->width;
		buffer.mode.v_width = buf->pitch / buffer.mode.cpp;
		buffer.mode.height = buf->height;
		buffer.mode.v_height = buf->height;
		buffer.mode.pitch = buf->pitch;

		buffer.funcs = FB_DEFAULT_FUNCS;

		// If no early framebuffer has been set previously, do it now.
		if (fb_get_early() == NULL)
		{
			fb_set_early(&buffer);
			terminal_init();
		}
		boot_kmesg("Early framebuffer: Address = 0x%p, Resolution = %ux%ux%hhu (Virtual = %ux%u)\n",
				   buffer.info.mmio_base, buffer.mode.width, buffer.mode.height, buffer.mode.cpp * 8,
				   buffer.mode.v_width, buffer.mode.v_height);
	}

	// Get kernel file.
	kassert(kernel_file_request.response, "Unable to get kernel file info!");
	struct limine_kernel_file_response* const kernel_res = kernel_file_request.response;
	boot_kmesg("Kernel file loaded at: 0x%p, Size = 0x%lx\n", kernel_res->kernel_file->address,
			   kernel_res->kernel_file->size);

	elf_set_kernel(kernel_res->kernel_file->address);

	// Get command line.
	boot_kmesg("Command line: \"%s\"\n", kernel_res->kernel_file->cmdline);
	info.cmd = kernel_res->kernel_file->cmdline;

#ifdef CONFIG_smp
	// Get SMP info
	kassert(smp_request.response, "Unable to get kernel SMP info!");
	const struct limine_smp_response* smp_res = smp_request.response;
	info.cpu_num = smp_res->cpu_count;
	boot_kmesg("Initializing %zu cores.\n", info.cpu_num);
	info.cpu_active = 0;
	info.boot_cpu = smp_res->bsp_lapic_id;	  // Mark the boot CPU.
	info.cpus = kzalloc(sizeof(Cpu) * info.cpu_num);
	for (usize i = 0; i < info.cpu_num; i++)
	{
		struct limine_smp_info* smp_cpu = smp_res->cpus[i];
		smp_cpu->extra_argument = (u64)&info.cpus[i];
		Cpu* cpu = &info.cpus[i];
		cpu->id = i;
#ifdef CONFIG_arch_x86
		cpu->lapic_id = smp_cpu->lapic_id;
		// Allocate stack.
		cpu->tss.rsp0 = pm_arch_alloc(CONFIG_stack_size / CONFIG_page_size) + (u64)pm_get_phys_base();
		cpu->tss.ist1 = pm_arch_alloc(CONFIG_stack_size / CONFIG_page_size) + (u64)pm_get_phys_base();
		cpu->tss.ist2 = cpu->tss.ist1;
#endif
		if (cpu->lapic_id != info.boot_cpu)
			smp_cpu->goto_address = limine_init_cpu;
		else
			limine_init_cpu(smp_cpu);
	}
	while (info.cpu_active != info.cpu_num)
	{
		asm_pause();
	}
	// From now on we have to use the spin lock mechanism to keep track of the current CPU.
	spin_use(true);
#endif
	boot_kmesg("Total processors active: %zu\n", info.cpu_active);

	boot_kmesg("HHDM offset: 0x%p\n", hhdm_request.response->offset);
	boot_kmesg("Kernel loaded at: 0x%p (0x%p)\n", kernel_address_request.response->virtual_base,
			   kernel_address_request.response->physical_base);

#ifdef CONFIG_acpi
	// Get ACPI RSDP.
	kassert(rsdp_request.response, "Unable to get ACPI RSDP!");
	boot_kmesg("ACPI RSDP at 0x%p\n", rsdp_request.response->address);
	info.acpi_rsdp = rsdp_request.response->address;
	kassert(memcmp(info.acpi_rsdp->signature, "RSD PTR", 7) == 0, "Invalid signature, expected \"RSD PTR\"!");
#endif

#ifdef CONFIG_open_firmware
	kassert(dtb_request.response, "Unable to get device tree!");
	boot_kmesg("FDT blob at 0x%p\n", dtb_request.response->dtb_ptr);
	info.fdt_blob = dtb_request.response->dtb_ptr;
#endif

	// Get modules.
	if (module_request.response == NULL)
		boot_kmesg("Unable to get modules, or none were provided!");
	else
	{
		boot_kmesg("Got modules:\n");
		const struct limine_module_response* module_res = module_request.response;
		BootFile* files = kmalloc(sizeof(BootFile) * module_res->module_count);
		for (usize i = 0; i < module_res->module_count; i++)
		{
			files[i].address = module_res->modules[i]->address;
			files[i].size = module_res->modules[i]->size;
			files[i].path = module_res->modules[i]->path;
			boot_kmesg("    [%i] Address = 0x%p, Size = 0x%zx, Path = \"%s\"\n", i, files[i].address, files[i].size,
					   files[i].path);
		}
		info.file_num = module_res->module_count;
		info.files = files;
	}

	arch_init(&info);
	// TODO: Swap out for call to scheduler.
	kernel_main(&info);
	arch_shutdown(&info);	 // Shut the system down safely.
	arch_stop(&info);		 // If we're still here, something went wrong. In that case, just try to stop.
}
