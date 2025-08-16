#ifndef _KERNEL_SCHED_TYPES_H
#define _KERNEL_SCHED_TYPES_H

#include <kernel/mem/types.h>
#include <bits/sched/types.h>
#include <stddef.h>
#include <stdint.h>

typedef size_t tid_t;

enum task_state {
    TASK_STATE_RUNNING,
    TASK_STATE_READY,
    TASK_STATE_BLOCKED,
};

struct task {
    tid_t id;
    struct process* process;
    enum task_state state;
    struct arch_context context;
    virt_t kernel_stack;
    virt_t user_stack;
    size_t time_slice;
    int8_t priority;
};

struct sched_percpu {
    struct task* current;
};

#endif
