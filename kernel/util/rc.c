#include <menix/refcount.h>

void rc_cleanup(void* r) {
	struct __refcount* rc = *(void**)r - sizeof(struct __refcount);
	rc->refcount--;
	if (rc->refcount == 0)
		kfree(rc);
}
