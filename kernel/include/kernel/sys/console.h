#ifndef _KERNEL_SYS_CONSOLE_H
#define _KERNEL_SYS_CONSOLE_H

#include <kernel/util/attributes.h>
#include <kernel/util/common.h>
#include <stddef.h>

// Defines a new early console which can be used to log early boot messages.
// An early console must not make any heap allocations.
#define DEFINE_EARLYCON(con) \
    [[__used, __section(".earlycon")]] \
    static struct console* UNIQUE_IDENT(__earlycon_) = &(con)

struct console {
    const char* name;
    void* private;

    // Initializes the console.
    void (*init)(struct console* con);
    // Writes to the console.
    void (*write)(struct console* con, const char* buf, size_t count);

    struct console* next;
};

void earlycon_init();

void console_add(struct console* con);
void console_remove(struct console* con);

void console_write(const char* buf, size_t len);

#endif
