#ifndef __MENIX_TYPES_H
#define __MENIX_TYPES_H

#include <menix/status.h>
#include <stdint.h>

// A generic object handle.
typedef uint32_t menix_handle_t;
typedef uint32_t menix_handle_type_t;

#define MENIX_HANDLE_INVALID   ((menix_handle_t)0)
#define MENIX_HANDLE_INIT_PORT ((menix_handle_t) - 1)

enum menix_port_flags {
    MENIX_PORT_FLAG_NONE = 0,
    // Allow sending messages even if one endpoint is not connected.
    MENIX_PORT_FLAG_ALLOW_UNCONNECTED = 1 << 0,
};

typedef menix_status_t (*menix_port_action_t)(menix_handle_t);

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

#endif
