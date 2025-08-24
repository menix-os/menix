#include <kernel/boot/cmdline.h>
#include <kernel/sys/console.h>
#include <string.h>

extern struct console* const __ld_earlycon_start[];
extern struct console* const __ld_earlycon_end[];

struct console* earlycon = nullptr;

void console_write(const char* buf, size_t len) {
    if (earlycon) {
        earlycon->write(earlycon, buf, len);
    }
}

void console_add(struct console* con) {}
void console_remove(struct console* con) {}

static void earlycon_setup(const char* value) {
    if (!value)
        return;

    // Find a suitable boot console.
    struct console* const* cur = __ld_earlycon_start;
    while (cur < __ld_earlycon_end) {
        if (!strcmp(value, (*cur)->name)) {
            earlycon = *cur;
        }
        cur++;
    }
}
CMDLINE_OPTION("earlycon", earlycon_setup);

void earlycon_init() {
    if (earlycon)
        earlycon->init(earlycon);
}
