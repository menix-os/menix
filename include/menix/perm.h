#ifndef _MENIX_CAPS_H
#define _MENIX_CAPS_H

typedef enum menix_perm {
    MENIX_PERM_NONE = 0,
    // Allows moving to another processes.
    MENIX_PERM_MOVE = 1 << 1,
    // Allows cloning the object handle.
    MENIX_PERM_CLONE = 1 << 2,
} menix_perm_t;

#endif
