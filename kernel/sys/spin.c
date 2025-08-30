#include <kernel/spin.h>

void spin_lock(struct spinlock* spin) {
    while (spin->locked) {}

    spin->locked = true;
}

void spin_unlock(struct spinlock* spin) {
    spin->locked = false;
}

bool spin_locked(struct spinlock* spin) {
    return spin->locked;
}
