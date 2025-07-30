#ifndef _MENIX_SYS_CPU_H
#define _MENIX_SYS_CPU_H

#include <menix/util/attributes.h>
#include <menix/proc/sched.h>
#include <bits/cpu.h>
#include <stddef.h>

struct cpu {
    struct cpu* self; // A pointer to this structure.
    size_t id;        // The logical ID of this CPU.
    virt_t kernel_stack;
    virt_t user_stack;

    struct sched_percpu sched; // Scheduler data.
    struct arch_cpu arch;      // Definitions for each architecture.

    __atomic bool online;  // Whether this CPU is initialized and active.
    __atomic bool present; // Whether this CPU is plugged in.
};

struct cpu* cpu_new();

#endif
