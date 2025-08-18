#ifndef _MENIX_MEMORY_H
#define _MENIX_MEMORY_H

#include <menix/handle.h>
#include <menix/status.h>

// Virtual memory flags.
enum menix_vm_flags {
    MENIX_VM_READ = 1 << 0,
    MENIX_VM_WRITE = 1 << 1,
    MENIX_VM_EXEC = 1 << 2,
    MENIX_VM_SHARED = 1 << 3,
};

enum menix_cache_type {
    // Generic memory
    MENIX_CACHE_NORMAL,
    // Write combining
    MENIX_CACHE_WC,
    // Memory-mapped IO
    MENIX_CACHE_MMIO,
};

#endif
