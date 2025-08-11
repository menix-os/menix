#ifndef _MENIX_SYS_OBJECT_H
#define _MENIX_SYS_OBJECT_H

#include <menix/posix/errno.h>
#include <menix/posix/types.h>
#include <stddef.h>

struct object;

struct object_ops {
    ssize_t (*read)();
    // Called before the object itself is freed.
    void (*free)(struct object* obj);
};

// A generic memory object.
// If you want to use it in a structure, make sure to put it first.
// To ensure this, use the `OBJECT` macro.
struct object {
    struct object* parent;
    const char* name;
    size_t ref_count;
    struct object_ops ops;
};

// Allocates a new object on the heap.
errno_t obj_new(size_t size, const char* name, void** out);
// Increases the refcount by 1.
void obj_ref_inc(struct object* obj);
// Decreases the refcount by 1.
void obj_ref_dec(struct object* obj);

#endif
