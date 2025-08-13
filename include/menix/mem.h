#ifndef _MENIX_MEM_H
#define _MENIX_MEM_H

enum menix_vm_flags {
    MENIX_VM_READ = 1 << 0,
    MENIX_VM_WRITE = 1 << 1,
    MENIX_VM_EXEC = 1 << 2,
    MENIX_VM_SHARED = 1 << 3,
};

enum menix_cache_type {
    MENIX_CACHE_NORMAL,
    MENIX_CACHE_WC,
    MENIX_CACHE_MMIO,
};

#endif
