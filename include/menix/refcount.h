#ifndef _MENIX_REFCOUNT_H
#define _MENIX_REFCOUNT_H

// This header provides basic

#include <menix/log.h>
#include <menix/alloc.h>

struct __refcount {
	long refcount;
};

#define rc(type) \
	struct { \
		struct __refcount __inner; \
		type __value; \
	}*

#define rc_new(default) \
	({ \
		rc(typeof(default)) __r = kmalloc(sizeof(*__r), 0); \
		__r->__inner.refcount = 0; \
		__r->__value = (default); \
		(void*)__r; \
	})

#define __rc [[gnu::cleanup(rc_cleanup)]]

#define rc_deref(r) \
	({ \
		(r)->__inner.refcount++; \
		&((r)->__value); \
	})

void rc_cleanup(void* r);

#endif
