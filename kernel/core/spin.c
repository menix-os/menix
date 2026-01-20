#include <menix/spin.h>
#include <stdatomic.h>

void spin_lock(struct spinlock* spin) {
    while (atomic_load_explicit(&spin->locked, memory_order_acquire)) {
        asm volatile("" ::: "memory");
    }

    atomic_store_explicit(&spin->locked, true, memory_order_release);
}

void spin_unlock(struct spinlock* spin) {
    spin->locked = false;
}
