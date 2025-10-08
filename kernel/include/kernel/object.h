#pragma once

#include <menix/status.h>
#include <stddef.h>

struct object;

// A generic object.
struct object {
    size_t ref_count;

    void (*drop)(struct object* obj);
};

// Allocates a new object on the heap.
menix_status_t obj_new(size_t size, const char* name, void** out);
// Increases the refcount by 1.
void obj_ref_inc(struct object* obj);
// Decreases the refcount by 1.
void obj_ref_dec(struct object* obj);

// A handle to an object.
struct object_handle {
    // The actual object.
    struct object* object;
};
