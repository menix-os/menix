#include <menix/boot/cmdline.h>
#include <menix/boot/file.h>
#include <menix/boot/init.h>
#include <menix/boot/main.h>
#include <menix/mem/pm.h>
#include <menix/mem/types.h>
#include <menix/sys/kprintf.h>
#include <menix/util/assert.h>
#include <menix/util/common.h>
#include <config.h>

#ifdef CONFIG_ACPI
#include <drivers/acpi/acpi.h>
#endif

#include "limine.h"

[[__initdata_sorted("limine.0")]] static volatile LIMINE_REQUESTS_START_MARKER;
[[__initdata_sorted("limine.1")]] static volatile LIMINE_BASE_REVISION(3);
[[__initdata_sorted("limine.2")]] static volatile LIMINE_REQUESTS_END_MARKER;

#define LIMINE_REQUEST(request, tag, rev) \
    [[__initdata_sorted("limine.1")]] \
    static volatile struct limine_##request request = { \
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
LIMINE_REQUEST(dtb_request, LIMINE_DTB_REQUEST, 0);

#ifdef CONFIG_ACPI
LIMINE_REQUEST(rsdp_request, LIMINE_RSDP_REQUEST, 0);
#endif

[[__init]]
void kernel_start() {
    ASSERT(executable_file_request.response, "Unable to get kernel file info!");
    boot_cmdline(executable_file_request.response->executable_file->string);

    ASSERT(hhdm_request.response, "Unable to get HHDM response!");
    ASSERT(executable_address_request.response, "Unable to get kernel address info!");

    ASSERT(memmap_request.response, "Unable to get memory map!");
    struct limine_memmap_response* const mm_res = memmap_request.response;

    // We create a VLA here because we have no other form of memory management at this point.
    // Assume that Limine provides sane values that don't overflow the stack.
    struct phys_mem mem_map[mm_res->entry_count];

    for (size_t i = 0; i < ARRAY_SIZE(mem_map); i++) {
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

    if (module_request.response == nullptr)
        pr_err("limine: Unable to get modules, or none were provided!\n");
    else {
        const struct limine_module_response* module_res = module_request.response;
        pr_log("limine: Got %zu module(s)\n", module_res->module_count);
        boot_files_count = MIN(module_res->module_count, ARRAY_SIZE(boot_files));
        for (size_t i = 0; i < boot_files_count; i++) {
            boot_files[i].data = module_res->modules[i]->address;
            boot_files[i].length = module_res->modules[i]->size;
            boot_files[i].path = module_res->modules[i]->path;
        }
    }

    [[__unused]]
    auto kernel_phys_base = (phys_t)executable_address_request.response->physical_base;
    [[__unused]]
    auto kernel_virt_base = (virt_t)executable_address_request.response->virtual_base;
    [[__unused]]
    auto hhdm_base = (virt_t)hhdm_request.response->offset;

#ifdef CONFIG_ACPI
    if (rsdp_request.response)
        acpi_rsdp_address = (phys_t)rsdp_request.response->address;
#endif

    kmain();
}
