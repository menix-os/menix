#ifndef _KERNEL_BITS_PERCPU_H
#define _KERNEL_BITS_PERCPU_H

#include <stddef.h>
#include <stdint.h>

#define arch_percpu_read(var) \
    ({ \
        typeof(((struct percpu*)nullptr)->var) __result; \
        asm volatile("mov %0, gs:%1" : "=r"(__result) : "i"(offsetof(struct percpu, var)) : "memory"); \
        __result; \
    })

#define arch_percpu_write(var, val) \
    ({ asm volatile("mov gs:%0, %1" ::"i"(offsetof(struct percpu, var)), "r"(val) : "memory"); })

#define arch_percpu_inc(var) ({ asm volatile("inc qword ptr gs:%0" ::"i"(offsetof(struct percpu, var)) : "memory"); })

#define arch_percpu_dec(var) ({ asm volatile("dec qword ptr gs:%0" ::"i"(offsetof(struct percpu, var)) : "memory"); })

struct arch_percpu {
    uint32_t lapic_id;
};

#endif
