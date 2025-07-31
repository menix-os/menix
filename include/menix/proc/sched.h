#ifndef _MENIX_PROC_SCHED_H
#define _MENIX_PROC_SCHED_H

#include <menix/mem/types.h>
#include <menix/posix/types.h>

#include <bits/sched.h>

enum task_state {
    TASK_STATE_RUNNING,
    TASK_STATE_READY,
    TASK_STATE_BLOCKED,
};

typedef size_t tid_t;

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
