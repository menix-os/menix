#pragma once

#include <menix/compiler.h>
#include <menix/list.h>
#include <menix/vector.h>
#include <bits/irq.h>
#include <stdint.h>

enum irq_status : uint8_t {
    IRQ_STATUS_HANDLED,
    IRQ_STATUS_IGNORED,
};

enum irq_polarity : uint8_t {
    IRQ_POLARITY_LOW,
    IRQ_POLARITY_HIGH,
};

enum irq_trigger_mode : uint8_t {
    IRQ_TRIGGER_EDGE,
    IRQ_TRIGGER_LEVEL,
};

typedef enum irq_status (*irq_handler_t)(void* self);

struct irq_line {
    int (*set_config)(enum irq_trigger_mode mode, enum irq_polarity polarity);
    void (*mask)(struct irq_line* self);
    void (*unmask)(struct irq_line* self);
    void (*eoi)(struct irq_line* self);

    SLIST_HEAD(irq_handler_t) handlers;
    bool is_busy;
};

void irq_line_attach(struct irq_line* self, irq_handler_t handler);

struct irq_percpu {
    uint32_t level;
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
