#ifndef _MENIX_ARCHCTL_H
#define _MENIX_ARCHCTL_H

#include <menix/compiler.h>

__MENIX_CDECL_START

typedef enum {
#ifdef __x86_64__
    MENIX_ARCHCTL_SET_FSBASE = 0,
#else
#error "Unsupported architecture!"
#endif
} menix_archctl_t;

__MENIX_CDECL_END

#endif
