#pragma once

#include <kernel/mem/types.h>

void mem_init(struct phys_mem* map, size_t length, virt_t kernel_virt, phys_t kernel_phys, virt_t hhdm_address);

// Returns a mapped address for the given physical address.
// If `phys` is not a valid address, returns `nullptr`.
void* mem_hhdm(phys_t phys);
