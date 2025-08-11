#include <menix/boot/cmdline.h>
#include <menix/sys/console.h>
#include <string.h>

extern struct console* __ld_bootcon_start[];
extern struct console* __ld_bootcon_end[];
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
    struct console** cur = __ld_bootcon_start;
    while (cur <= __ld_bootcon_end) {
        if (!strncmp(value, (*cur)->name, strlen(value))) {
            bootcon = *cur;
            bootcon->init(bootcon);
        }
        cur++;
    }
}
CMDLINE_OPTION("bootcon", bootcon_setup);
