#include <kernel/irq.h>
#include <kernel/percpu.h>

void irq_lock() {
    percpu_inc(irq.level);
}

void irq_unlock() {}
