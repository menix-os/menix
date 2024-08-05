// Spinlock

#pragma once

#include <menix/common.h>

typedef struct
{
	void* owner;	// The most recent owner.
	usize cpu;		// The CPU ID connected to the owner.
	bool locked;	// Whether it's locked or not.
} SpinLock;

// Attempt to acquire the lock.
// Returns true if successful.
bool spin_acquire(SpinLock* lock);

// Attempt to acquire the lock.
// If unsuccessful, attempts again.
void spin_acquire_force(SpinLock* lock);

// Frees the lock if it was previously locked.
void spin_free(SpinLock* lock);
