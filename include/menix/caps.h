#ifndef _MENIX_CAPS_H
#define _MENIX_CAPS_H

enum menix_cap {
    MENIX_CAP_NONE = 0,
    MENIX_CAP_READ = 1 << 0,
    MENIX_CAP_WRITE = 1 << 1,
    MENIX_CAP_EXEC = 1 << 2,
    MENIX_CAP_MOVE = 1 << 3,
    MENIX_CAP_CLONE = 1 << 4,
};

#endif
