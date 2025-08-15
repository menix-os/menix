#ifndef _MENIX_OBJECT_H
#define _MENIX_OBJECT_H

#include <menix/status.h>
#include <stdint.h>

// A generic object handle.
typedef uint32_t menix_obj_t;

enum : menix_obj_t {
    // A handle value of 0 is always invalid.
    MENIX_HANDLE_NULL = 0,
};

enum menix_rights {
    MENIX_RIGHT_NONE = 1 << 0,
    // Object handle may be moved to another processes.
    MENIX_RIGHT_MOVE = 1 << 1,
    // Object handle may be cloned.
    MENIX_RIGHT_CLONE = 1 << 2,
};

#ifndef __KERNEL__

// Checks an object handle for validity.
menix_status_t menix_handle_check(menix_obj_t object);

// Closes an object handle.
menix_status_t menix_handle_close(menix_obj_t object);

// Clones an object handle.
menix_status_t menix_handle_clone(menix_obj_t object, menix_obj_t* cloned);

#endif
#endif
