#include <kernel/mem/mm.h>
#include <kernel/mem/paging.h>
#include <kernel/mem/types.h>
#include <kernel/print.h>

extern uint8_t __ld_text_start[];
extern uint8_t __ld_text_end[];
extern uint8_t __ld_rodata_start[];
extern uint8_t __ld_rodata_end[];
extern uint8_t __ld_data_start[];
extern uint8_t __ld_data_end[];
extern uint8_t __ld_kernel_start[];

static virt_t hhdm_base = 0;
static struct page_table kernel_page_table = {0};

void* mem_hhdm(phys_t phys) {
    return (void*)(phys + hhdm_base);
}

void mem_init(struct phys_mem* map, size_t length, virt_t kernel_virt, phys_t kernel_phys, virt_t hhdm_address) {
    // Print the memory map.
    kprintf("Memory map:\n");
    for (size_t i = 0; i < length; i++) {
        if (map[i].length > 0 && map[i].usage == PHYS_USABLE)
            kprintf("\t[%p - %p]\n", (void*)map[i].address, (void*)(map[i].address + map[i].length - 1));
    }

    for (uint8_t* p = __ld_text_start; p < __ld_text_end; p += mem_page_size()) {
        mem_pt_map(
            &kernel_page_table,
            (virt_t)p,
            (phys_t)(__ld_text_start - __ld_kernel_start + kernel_phys),
            PTE_READ | PTE_EXEC
        );
    }
}
