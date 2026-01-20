#ifndef MENIX_UAPI_ARCHCTL_H
#define MENIX_UAPI_ARCHCTL_H

typedef enum {
#ifdef __x86_64__
    MENIX_ARCHCTL_SET_FSBASE = 0,
#endif
} menix_archctl_t;

#endif
