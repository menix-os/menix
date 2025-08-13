#ifndef _MENIX_STATUS_H
#define _MENIX_STATUS_H

#include <stdint.h>

// A status value of 0 means everything is okay.
// A negative value indicates an error from the kernel.
// A positive value indicates an error from a server.
typedef int32_t menix_status_t;

// Kernel errors.
enum : menix_status_t {
    MENIX_OK = 0,
    // An internal error occured.
    MENIX_ERR_INTERNAL,
    // This operation is not supported.
    MENIX_ERR_UNSUPPORTED,
    // System does not have enough free memory for this operation.
    MENIX_ERR_NO_MEMORY,
    // Process can not allocate any more handles.
    MENIX_ERR_NO_HANDLES,
    // Syscall number is not a recognized syscall.
    MENIX_ERR_BAD_SYSCALL,
    // One or more of the provided arguments is not valid.
    MENIX_ERR_BAD_ARG,
    // Argument is out of range for valid values.
    MENIX_ERR_BAD_RANGE,
    // Object handle does not name a valid object.
    MENIX_ERR_BAD_OBJECT,
    // Object handle is valid, but names an incorrect object type.
    MENIX_ERR_BAD_OBJECT_TYPE,
    // Object has insufficient capabilites for this operation.
    MENIX_ERR_BAD_CAPS,
    // Buffer is not large enough or doesn't point to a valid memory region.
    MENIX_ERR_BAD_BUFFER,
};

#endif
