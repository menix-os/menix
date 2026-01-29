#ifndef MENIX_MEMORY_H
#define MENIX_MEMORY_H

#include <menix/errno.h>
#include <menix/handle.h>

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

typedef uintptr_t menix_virt_t;
typedef uintptr_t menix_phys_t;

#ifndef __KERNEL__
#include <stddef.h>

menix_errno_t menix_mem_alloc(size_t length, menix_handle_t* out);

#endif

#endif
