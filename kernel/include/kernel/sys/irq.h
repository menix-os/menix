#pragma once

#include <kernel/compiler.h>
#include <bits/irq.h>
#include <stdint.h>

struct irq_percpu {
    __atomic(uint32_t) level;
};

void irq_lock();

void irq_unlock();

// Enables or disables interrupts on this CPU.
// Don't use this function directly.
static inline void irq_set_state(bool state) {
    return arch_irq_set_state(state);
}

// Returns true if interrupts are enabled on this CPU.
// Don't use this function directly.
static inline bool irq_get_state() {
    return arch_irq_get_state();
}
