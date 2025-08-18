#ifndef _KERNEL_BITS_SYS_PERCPU_H
#define _KERNEL_BITS_SYS_PERCPU_H

struct arch_percpu {};

#define arch_percpu_read(var)       ({ ((__seg_gs struct percpu*)nullptr)->var; })
#define arch_percpu_write(var, val) ({ ((__seg_gs struct percpu*)nullptr)->var = val; })

#endif
