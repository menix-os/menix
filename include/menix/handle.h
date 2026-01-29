#ifndef MENIX_HANDLE_H
#define MENIX_HANDLE_H

#include <menix/errno.h>
#include <menix/rights.h>
#include <stdint.h>

// A generic object handle.
typedef uint32_t menix_handle_t;

#define MENIX_HANDLE_INVALID   ((menix_handle_t)(0))
#define MENIX_HANDLE_INIT_PORT ((menix_handle_t)(-1))

#ifndef __KERNEL__

// Checks an object handle for validity.
menix_errno_t menix_handle_validate(menix_handle_t object);

// Drops a an object handle.
// All further references using this handle are invalid. The numerical value
// may become valid again, but it is an error to keep using it.
menix_errno_t menix_handle_drop(menix_handle_t handle);

// Clones an object handle.
menix_errno_t menix_handle_clone(menix_handle_t object, menix_rights_t cloned_rights, menix_handle_t* cloned);

#endif

#endif
