// Mutual exclusion spin-lock

#pragma once
#include <menix/common.h>

#include <stdatomic.h>

// Creates a mutex and makes sure it's automatically released upon scope exit.
#define MUTEX_LOCK()

typedef struct
{
	_Atomic usize active;
	_Atomic usize next;
} Mutex;

// Locks a mutex.
static inline void mutex_lock(Mutex* mutex)
{
	// Wait until it's our turn on the mutex.
	usize this_id = atomic_fetch_add(&mutex->next, 1);
	while (atomic_load(&mutex->active) != this_id)
		;
}

// Unlocks a mutex.
static inline void mutex_unlock(Mutex* mutex)
{
	atomic_fetch_add(&mutex->active, 1);
}

// Initializes a new mutex lock.
static inline void mutex_create(Mutex* mutex)
{
	atomic_init(&mutex->active, 0);
	atomic_init(&mutex->next, 0);
}

// Invalidates a mutex lock.
static inline void mutex_destroy(Mutex* mutex)
{
	mutex_unlock(mutex);
}
