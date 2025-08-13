#ifndef _KERNEL_PROC_SCHED_H
#define _KERNEL_PROC_SCHED_H

#include <kernel/mem/types.h>
#include <bits/sched.h>
#include <stddef.h>

typedef size_t tid_t;
typedef size_t pid_t;

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

struct process {
    pid_t id;
    const char* name;
    struct process* parent;
};

struct sched_percpu {
    struct task* current;
};

#endif
