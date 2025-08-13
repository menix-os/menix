#include <kernel/boot/cmdline.h>
#include <kernel/sys/console.h>
#include <string.h>

extern struct console* const __ld_earlycon_start[];
extern struct console* const __ld_earlycon_end[];

struct console* bootcon = nullptr;

void console_write(const char* buf, size_t len) {
    if (bootcon) {
        bootcon->write(bootcon, buf, len);
    }
}

void console_add(struct console* con) {}
void console_remove(struct console* con) {}

static void bootcon_setup(const char* value) {
    if (!value)
        return;

    // Find a suitable boot console.
    struct console* const* cur = __ld_earlycon_start;
    while (cur < __ld_earlycon_end) {
        if (!strncmp(value, (*cur)->name, strlen(value))) {
            bootcon = *cur;
            bootcon->init(bootcon);
        }
        cur++;
    }
}
CMDLINE_OPTION("bootcon", bootcon_setup);
