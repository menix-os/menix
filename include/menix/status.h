#ifndef _MENIX_STATUS_H
#define _MENIX_STATUS_H

#include <menix/compiler.h>
#include <stdint.h>

__MENIX_CDECL_START

// A status value of 0 means everything is okay.
// A negative value indicates an error from the kernel.
// Positive values are available to user processes.
typedef int32_t menix_status_t;

// Status values that may be returned by the kernel.
enum {
    MENIX_OK = 0,
    // An internal error occured.
    MENIX_ERR_INTERNAL,
    // Syscall number is not a recognized syscall.
    MENIX_ERR_BAD_SYSCALL,
    // This operation is not supported.
    MENIX_ERR_UNSUPPORTED,
    // System does not have enough free memory for this operation.
    MENIX_ERR_NO_MEMORY,
    // Process can not own any more handles.
    MENIX_ERR_NO_HANDLES,
    // One or more of the provided arguments is not valid.
    MENIX_ERR_BAD_ARG,
    // Argument is outside of the range for valid values.
    MENIX_ERR_BAD_RANGE,
    // Object handle does not name a valid object.
    MENIX_ERR_BAD_OBJECT,
    // Object handle is valid, but names an incorrect object type.
    MENIX_ERR_BAD_OBJECT_TYPE,
    // Object has insufficient permissions for this operation.
    MENIX_ERR_BAD_PERMS,
    // Buffer is not large enough or doesn't point to a valid memory region.
    MENIX_ERR_BAD_BUFFER,
};

__MENIX_CDECL_END

#endif
