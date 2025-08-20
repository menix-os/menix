#ifndef _KERNEL_IRQ_H
#define _KERNEL_IRQ_H

#include <kernel/arch/irq.h>
#include <kernel/compiler.h>
#include <stdint.h>

struct irq_percpu {
    __atomic(uint32_t) level;
};

// Disables interrupts.
void irq_lock();

// Enables interrupts
void irq_unlock();

#endif
