#pragma once

#include <stdatomic.h>

// Busy-waits in a loop until the lock is freed.
// Does not put the CPU to sleep.
struct spinlock {
    atomic bool locked;
};

void spin_lock(struct spinlock* spin);
void spin_unlock(struct spinlock* spin);
