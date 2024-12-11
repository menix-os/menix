// Spinlock

#pragma once

#include <menix/common.h>

typedef struct
{
	void* owner;	// The most recent owner.
	usize cpu;		// The CPU ID connected to the owner.
	bool locked;	// Whether it's locked or not.
} SpinLock;

#define spin_lock_scope(lock, scope) \
	spin_lock(lock); \
	do \
		scope while (0); \
	spin_unlock(lock);

// Toggles if spinlocks do anything or not. Used for single processor machines/during setup.
void spin_use(bool on);

// Attempt to acquire the lock.
// Returns true if successful.
bool spin_try_lock(SpinLock* lock);

// Attempt to acquire the lock.
// If unsuccessful, attempts again.
void spin_lock(SpinLock* lock);

// Frees the lock if it was previously locked.
void spin_unlock(SpinLock* lock);
