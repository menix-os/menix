#ifndef _MENIX_OBJECT_H
#define _MENIX_OBJECT_H

#include <menix/status.h>
#include <stdint.h>

#define MENIX_OBJECT_INVALID ((menix_object_t)0)

typedef uint32_t menix_object_t;

#ifndef __KERNEL__

// Checks the object handle for validity.
menix_status_t menix_object_check(menix_object_t object);

// Closes the object handle.
menix_status_t menix_object_close(menix_object_t object);

// Clones the object handle.
menix_status_t menix_object_clone(menix_object_t object, menix_object_t* out);

#endif
#endif
