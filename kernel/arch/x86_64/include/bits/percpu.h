#pragma once

#include <stdint.h>
#include <x86_64/gdt.h>

static inline struct percpu* arch_percpu_get() {
    struct percpu* __result;
    asm volatile("mov %0, gs:0" : "=r"(__result)::"memory");
    return __result;
}

struct arch_percpu {
    uint32_t lapic_id;
    struct gdt gdt;
    struct tss tss;
};
