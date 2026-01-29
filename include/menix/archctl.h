#ifndef MENIX_ARCHCTL_H
#define MENIX_ARCHCTL_H

typedef enum {
    MENIX_ARCHCTL_NONE = 0,
#if defined(__x86_64__)
    MENIX_ARCHCTL_SET_FSBASE = 1,
#endif
} menix_archctl_t;

#ifndef __KERNEL__

// Performs an architecture-dependent operation identified by `op`.
menix_errno_t menix_archctl(menix_archctl_t op, size_t value);

#endif

#endif
