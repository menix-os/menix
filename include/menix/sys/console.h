#ifndef _MENIX_SYS_CONSOLE_H
#define _MENIX_SYS_CONSOLE_H

#include <menix/util/attributes.h>
#include <stddef.h>

struct console {
    const char* name;
    void* private;

    // Writes to the console.
    void (*write)(struct console* con, const char* buf, size_t count);
    // Reads from the console.
    size_t (*read)(struct console* con, char* buf, size_t count);
    // Sets up the console.
    int (*setup)(struct console* con, char* options);
};

#endif
