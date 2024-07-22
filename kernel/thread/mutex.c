#include <menix/log.h>
#include <menix/thread/mutex.h>

void mutex_create(Mutex* mutex)
{
	kassert(mutex != NULL, "No mutex given!");
}

void mutex_destroy(Mutex* mutex)
{
}

void mutex_lock(Mutex* mutex)
{
}

void mutex_unlock(Mutex* mutex)
{
}
