#include <kernel/spin.h>
#include <stdatomic.h>
#include <stdint.h>

void spin_lock(struct spinlock* spin) {
    while (atomic_exchange_explicit(&spin->locked, true, memory_order_acquire)) {
        asm volatile("pause");
    }

    atomic_store_explicit(&spin->locked, true, memory_order_release);
}

void spin_unlock(struct spinlock* spin) {
    atomic_store_explicit(&spin->locked, false, memory_order_release);
}

#include <stddef.h>

struct foo {
    void* atomic foo;
};

void asd(struct foo* f) {
    atomic_exchange(&f->foo, nullptr);
}
