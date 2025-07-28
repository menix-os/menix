#ifndef _MENIX_SCHED_H
#define _MENIX_SCHED_H

#include <menix/types.h>
#include <menix/mem_types.h>
#include <menix/posix_types.h>
#include <bits/sched.h>

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
	usize time_slice;
	i8 priority;
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
