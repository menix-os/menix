#include <kernel/irq.h>
#include <kernel/percpu.h>

void irq_lock() {
    irq_set_state(false);
    percpu_get()->irq.level++;
}

void irq_unlock() {
    __atomic(uint32_t) old_level = percpu_get()->irq.level--;
    // If it was 1, the new IRQ level is now 0.
    if (old_level == 1) {
        irq_set_state(true);
    }
}
