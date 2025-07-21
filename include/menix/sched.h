#ifndef _MENIX_SCHED_H
#define _MENIX_SCHED_H

#include <uapi/posix/types.h>
#include <menix/types.h>
#include <menix/mem.h>
#include <bits/sched.h>

typedef __tid_t tid_t;
typedef __pid_t pid_t;

enum thread_state {
	THREAD_STATE_RUNNING,
	THREAD_STATE_READY,
	THREAD_STATE_BLOCKED,
};

struct thread {
	tid_t id;
	struct process* process;
	enum thread_state state;
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

#endif
