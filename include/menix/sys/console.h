#ifndef _MENIX_SYS_CONSOLE_H
#define _MENIX_SYS_CONSOLE_H

#include <menix/util/attributes.h>

#include <stddef.h>

struct console {
    char name[16];
    void (*write)(struct console* con, const char* buf, size_t count);
    size_t (*read)(struct console* con, char* buf, size_t count);
    int (*init)(struct console* con, char* options);
};

#endif
