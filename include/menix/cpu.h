#ifndef _MENIX_CPU_H
#define _MENIX_CPU_H

#include <menix/mem.h>
#include <menix/types.h>
#include <menix/sched.h>
#include <bits/cpu.h>

#define this_cpu_read(field)		 arch_percpu_read(field)
#define this_cpu_write(field, value) arch_percpu_write(field, value)

struct cpu {
	struct cpu* self; // A pointer to this structure.
	usize id;		  // The logical ID of this CPU.
	virt_t kernel_stack;
	virt_t user_stack;

	struct sched_percpu sched;

	bool online;
	bool present;

	struct arch_cpu arch;
};

struct cpu* cpu_new();

#endif
