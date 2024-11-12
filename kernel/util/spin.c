// Spinlock implementation

#include <menix/system/arch.h>
#include <menix/util/spin.h>

static void* last_addr = NULL;
static bool use_spin = true;

void spin_use(bool on)
{
	use_spin = on;
}

bool spin_acquire(SpinLock* lock)
{
	if (!use_spin)
		return true;

	if (!lock)
		return false;

	// Was locked set before? If no, set it to 1.
	bool result = __sync_bool_compare_and_swap(&lock->locked, 0, 1);

	if (result)
	{
		lock->owner = __builtin_return_address(0);
	}

	return result;
}

void spin_acquire_force(SpinLock* lock)
{
	if (!lock)
		return;

	last_addr = __builtin_return_address(0);

	// Keep trying to lock.
	while (1)
	{
		if (spin_acquire(lock))
			break;
		asm_pause();
	}

	lock->owner = __builtin_return_address(0);
}

void spin_free(SpinLock* lock)
{
	if (!use_spin)
		return;

	if (!lock)
		return;

	__atomic_store_n(&lock->locked, 0, __ATOMIC_SEQ_CST);
}
