#ifndef _KERNEL_OBJECT_H
#define _KERNEL_OBJECT_H

#include <menix/status.h>
#include <stddef.h>

struct object;

// A generic object.
struct object {
    size_t ref_count;

    // Called to close an object.
    void (*close)(struct object* obj);
};

// Allocates a new object on the heap.
menix_status_t obj_new(size_t size, const char* name, void** out);
// Increases the refcount by 1.
void obj_ref_inc(struct object* obj);
// Decreases the refcount by 1.
void obj_ref_dec(struct object* obj);

// A handle to a object.
struct object_handle {
    // The actual object.
    struct object* object;
};

#endif
