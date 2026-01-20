#pragma once

#include <menix/mem.h>
#include <menix/types.h>
#include <bits/sched.h>
#include <stddef.h>
#include <stdint.h>

ASSERT_TYPE(struct arch_context);
ASSERT_TYPE(struct arch_task_context);

typedef size_t tid_t;

enum task_state {
    TASK_STATE_RUNNING, // Task is active and running.
    TASK_STATE_READY,   // Task is active, but not currently scheduled.
    TASK_STATE_BLOCKED, // Task is waiting on another object.
};

// A task is the smallest unit of the scheduler.
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

// Per-CPU data for scheduling.
struct sched_percpu {
    struct task* current;
};

void sched_init();
