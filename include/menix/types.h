#ifndef _MENIX_TYPES_H
#define _MENIX_TYPES_H

#include <menix/compiler.h>
#include <stddef.h>
#include <stdint.h>

__MENIX_CDECL_START

// A generic object handle.
typedef uint32_t menix_handle_t;
typedef uint32_t menix_handle_type_t;

#define MENIX_HANDLE_INVALID ((menix_handle_t)0)

enum menix_port_flags {
    MENIX_PORT_FLAG_NONE = 0,
    // Allow sending messages even if one endpoint is not connected.
    MENIX_PORT_FLAG_ALLOW_UNCONNECTED = 1 << 0,
};

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

__MENIX_CDECL_END

#endif
