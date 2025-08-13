#ifndef _KERNEL_SYS_PERCPU_H
#define _KERNEL_SYS_PERCPU_H

#include <kernel/mem/types.h>
#include <kernel/util/attributes.h>
#include <stddef.h>

#define DEFINE_PERCPU(x)

// The start of every per-CPU region.
struct percpu {
    struct percpu* self;   // A pointer to this structure.
    size_t id;             // The logical ID of this CPU.
    virt_t kernel_stack;   // The kernel mode stack.
    virt_t user_stack;     // The user mode stack.
    __atomic bool online;  // Whether this CPU is initialized and active.
    __atomic bool present; // Whether this CPU is plugged in.
};

// Allocates a block of memory for a new CPU.
struct percpu* percpu_new();

#endif
