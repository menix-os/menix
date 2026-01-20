#pragma once

#include <menix/sched.h>
#include <stdatomic.h>

struct mutex {
    struct task* owner;
    atomic bool flag;
};

void mutex_lock(struct mutex* mutex);
void mutex_unlock(struct mutex* mutex);
