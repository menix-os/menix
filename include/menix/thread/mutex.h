//? Mutual exclusion spin-lock

#pragma once
#include <menix/common.h>

typedef struct
{
	void* memory;
} Mutex;

// Initializes a new mutex lock.
void mutex_create(Mutex* mutex);
// Invalidates a mutex lock.
void mutex_destroy(Mutex* mutex);
// Locks a mutex.
void mutex_lock(Mutex* mutex);
// Unlocks a mutex.
void mutex_unlock(Mutex* mutex);
