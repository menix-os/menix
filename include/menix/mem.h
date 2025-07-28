#ifndef _MENIX_MEM_H
#define _MENIX_MEM_H

#include <menix/types.h>
#include <menix/mem_types.h>

extern struct page mem_pfndb[];

extern struct phys_mem mem_map[128];
extern usize mem_map_size;
extern phys_t mem_kernel_phys_base;
extern virt_t mem_kernel_virt_base;
extern virt_t mem_hhdm_base;

void mem_init();

#endif
