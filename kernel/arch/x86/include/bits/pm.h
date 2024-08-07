// x86-specific physical memory management.

#pragma once

#include <menix/common.h>

// Allocates a given `amount` of `CONFIG_page_size` sized pages.
PhysAddr pm_arch_alloc(usize amount);

// Frees pages pointed to by `pm_arch_alloc`.
void pm_arch_free(PhysAddr addr);
