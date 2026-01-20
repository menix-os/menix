#pragma once

typedef enum {
#ifdef __x86_64__
    MENIX_ARCHCTL_SET_FSBASE = 0,
#endif
} menix_archctl_t;
