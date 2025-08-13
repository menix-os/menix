#ifndef _KERNEL_SYS_OBJECT_H
#define _KERNEL_SYS_OBJECT_H

#include <menix/status.h>
#include <stddef.h>

enum object_type {
    OBJECT_TYPE_NONE = 0,
    OBJECT_TYPE_
};

struct object;

struct object_ops {
    // Called to close an object.
    void (*close)(struct object* obj);
};

// A generic memory object.
struct object {
    struct object* parent;
    const char* name;
    size_t ref_count;
    struct object_ops ops;
};

// Allocates a new object on the heap.
menix_status_t obj_new(size_t size, const char* name, void** out);
// Increases the refcount by 1.
void obj_ref_inc(struct object* obj);
// Decreases the refcount by 1.
void obj_ref_dec(struct object* obj);

#endif
