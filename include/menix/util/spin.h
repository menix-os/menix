// Spinlock

#pragma once

#include <menix/common.h>

typedef struct
{
	void* owner;	// The most recent owner.
	usize cpu;		// The CPU ID connected to the owner.
	bool locked;	// Whether it's locked or not.
} SpinLock;

#define spin_new() \
	(SpinLock) \
	{ \
		0 \
	}

#define spin_lock(lock, scope) \
	spin_acquire_force(lock); \
	do \
		scope while (0); \
	spin_free(lock);

// Toggles if spinlocks do anything or not. Used for single processor machines/during setup.
void spin_use(bool on);

// Attempt to acquire the lock.
// Returns true if successful.
bool spin_acquire(SpinLock* lock);

// Attempt to acquire the lock.
// If unsuccessful, attempts again.
void spin_acquire_force(SpinLock* lock);

// Frees the lock if it was previously locked.
void spin_free(SpinLock* lock);
