#ifndef _KERNEL_SYS_PERCPU_H
#define _KERNEL_SYS_PERCPU_H

#include <kernel/mem/types.h>
#include <kernel/util/compiler.h>
#include <bits/sys/percpu.h>
#include <stddef.h>

#define percpu_read(x)       arch_percpu_read(x)
#define percpu_write(x, val) arch_percpu_write(x, val)

// The start of every per-CPU region.
struct percpu {
    struct percpu* self; // A pointer to this structure.
    size_t id;           // The logical ID of this CPU.
    virt_t kernel_stack; // The kernel mode stack.
    virt_t user_stack;   // The user mode stack.

    bool online; // Whether this CPU is initialized and active.
};

extern struct percpu percpu_bsp;

// Allocates a block of memory for a new CPU.
struct percpu* percpu_new();

#endif
