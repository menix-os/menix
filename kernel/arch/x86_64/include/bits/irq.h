#pragma once

#include <defs.h>
#include <stdint.h>

static inline bool arch_irq_get_state() {
    uint64_t flags;
    asm volatile("pushf; pop %0" : "=r"(flags)::"memory");
    return flags & RFLAGS_IF;
}

static inline void arch_irq_set_state(bool state) {
    if (state)
        asm volatile("sti");
    else
        asm volatile("cli");
}
