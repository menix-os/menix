// Spinlock implementation

#include <menix/arch.h>
#include <menix/thread/spin.h>

static void* last_addr = NULL;

bool spin_lock(SpinLock* lock)
{
	if (!lock)
		return false;

	// Was locked set before? If no, set it to 1.
	bool result = __sync_bool_compare_and_swap(&lock->locked, 0, 1);

	if (result)
		lock->owner = __builtin_return_address(0);

	return result;
}

void spin_lock_loop(SpinLock* lock)
{
	if (!lock)
		return;

	last_addr = __builtin_return_address(0);

	// Keep trying to lock.
	while (1)
	{
		if (spin_lock(lock))
			break;
		asm_pause();
	}

	lock->owner = __builtin_return_address(0);

#ifdef CONFIG_smp
	lock->cpu = arch_current_cpu()->id;
#endif
}

void spin_free(SpinLock* lock)
{
	if (!lock)
		return;

	__atomic_store_n(&lock->locked, 0, __ATOMIC_SEQ_CST);
}
