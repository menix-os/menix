#ifndef _KERNEL_ARCH_PERCPU_H
#define _KERNEL_ARCH_PERCPU_H

#include <kernel/common.h>
#include <bits/percpu.h>

ASSERT_TYPE(struct arch_percpu);

#define percpu_read(field)       arch_percpu_read(field)
#define percpu_write(field, val) arch_percpu_write(field, val)
#define percpu_inc(field)        arch_percpu_inc(field)
#define percpu_dec(field)        arch_percpu_dec(field)

#endif
