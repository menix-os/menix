#ifndef _MENIX_PERCPU_H
#define _MENIX_PERCPU_H

#include <menix/mem.h>
#include <menix/types.h>
#include <menix/sched.h>
#include <bits/percpu.h>

#define percpu_read(field)		   __percpu_read(field)
#define percpu_write(field, value) __percpu_write(field, value)

struct percpu {
	// A pointer to this structure.
	struct percpu* self;
	// The logical ID of this CPU.
	usize id;
	virt_t kernel_stack;
	virt_t user_stack;

	struct sched_percpu sched;

	bool online;
	bool present;

	struct arch_percpu arch;
};

struct percpu* percpu_new();

#endif
