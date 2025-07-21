#ifndef _MENIX_MM_TYPES_H
#define _MENIX_MM_TYPES_H

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
	usize refcount;
	union {
		struct {
			struct page* next;
			usize length;
		} freelist;
	};
};
static_assert(0x1000 % sizeof(struct page) == 0, "must be a multiple of the page size!");

#endif
