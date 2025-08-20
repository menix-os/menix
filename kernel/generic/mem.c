#include <kernel/mem.h>
#include <kernel/print.h>

extern uint8_t __ld_text_start[];
extern uint8_t __ld_text_end[];
extern uint8_t __ld_rodata_start[];
extern uint8_t __ld_rodata_end[];
extern uint8_t __ld_data_start[];
extern uint8_t __ld_data_end[];
extern uint8_t __ld_kernel_start[];

void mem_init(struct phys_mem* map, size_t length, virt_t kernel_virt, phys_t kernel_phys, virt_t hhdm_address) {
    kprintf("Memory map:\n");
    for (size_t i = 0; i < length; i++) {
        if (map[i].length > 0 && map[i].usage == PHYS_USABLE)
            kprintf("\t[%p - %p]\n", (void*)map[i].address, (void*)(map[i].address + map[i].length - 1));
    }
}
