#ifndef __MENIX_ARCHCTL_H
#define __MENIX_ARCHCTL_H

typedef enum {
#ifdef __x86_64__
    MENIX_ARCHCTL_SET_FSBASE = 0,
#else
#error "Unsupported architecture!"
#endif
} menix_archctl_t;

#endif
