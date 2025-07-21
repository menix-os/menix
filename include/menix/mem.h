#ifndef _MENIX_MEM_H
#define _MENIX_MEM_H

#include <menix/types.h>

typedef uptr phys_t;
typedef uptr virt_t;

enum phys_mem_usage {
	PHYS_RESERVED,
	PHYS_USABLE,
	PHYS_RECLAIMABLE,
};

struct phys_mem {
	phys_t address;
	usize length;
	enum phys_mem_usage usage;
};

struct page {
	usize flags;
	usize count;
	union {
		struct {
			struct page* next;
			usize length;
		} freelist;
	};
};
static_assert(0x1000 % sizeof(struct page) == 0, "must be a multiple of the page size!");

extern struct phys_mem mem_map[128];
extern usize mem_map_size;
extern phys_t mem_kernel_phys_base;
extern virt_t mem_kernel_virt_base;
extern virt_t mem_hhdm_base;

#endif
