#ifndef _MENIX_SYS_CONSOLE_H
#define _MENIX_SYS_CONSOLE_H

#include <menix/util/attributes.h>
#include <menix/util/common.h>
#include <stddef.h>

// Defines a new boot console which can be used to log early boot messages.
// These consoles don't need to be added explicitly.
#define DEFINE_BOOTCON(con) \
    [[__section(".bootcon")]] \
    static struct console* UNIQUE_IDENT(__bootcon_) = &(con)

struct console {
    const char* name;
    void* private;

    // Writes to the console.
    void (*write)(struct console* con, const char* buf, size_t count);

    // Initializes the console.
    void (*init)(struct console* con);

    struct console* next;
};

void console_add(struct console* con);
void console_remove(struct console* con);
void console_write(const char* buf, size_t len);

#endif
