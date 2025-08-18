#ifndef _MENIX_HANDLE_H
#define _MENIX_HANDLE_H

#include <menix/status.h>
#include <stdint.h>

// A generic object handle.
typedef uint32_t menix_handle_t;

enum menix_rights {
    MENIX_RIGHT_NONE = 1 << 0,
    // Object handle may be moved to another processes.
    MENIX_RIGHT_MOVE = 1 << 1,
    // Object handle may be cloned.
    MENIX_RIGHT_CLONE = 1 << 2,
};

#endif
