#pragma once

#include <kernel/compiler.h>

// Very simple synchronization primitive which waits until the lock is freed.
struct spinlock {
    __atomic(bool) locked;
};

void spin_lock(struct spinlock* spin);
void spin_unlock(struct spinlock* spin);
bool spin_locked(struct spinlock* spin);
