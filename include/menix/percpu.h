#ifndef _MENIX_PERCPU_H
#define _MENIX_PERCPU_H

#include <menix/types.h>
#include <bits/percpu.h>

#define percpu_read(field)		   __percpu_read(field)
#define percpu_write(field, value) __percpu_write(field, value)

struct percpu {
	usize id;
	struct arch_percpu arch;
};

#endif
