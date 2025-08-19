#ifndef _KERNEL_BITS_PERCPU_H
#define _KERNEL_BITS_PERCPU_H

#include <stdint.h>

#if defined(__clang__) || defined(__GNUC__)
#define arch_percpu_read(var)       ({ ((__seg_gs struct percpu*)nullptr)->var; })
#define arch_percpu_write(var, val) ({ ((__seg_gs struct percpu*)nullptr)->var = val; })
#else
#error "TODO: __seg_gs is only supported on clang and gcc"
#endif

struct arch_percpu {
    uint32_t lapic_id;
};

#endif
