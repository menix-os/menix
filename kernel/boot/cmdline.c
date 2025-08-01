#include <menix/boot/cmdline.h>
#include <menix/sys/kprintf.h>
#include <stddef.h>
#include <string.h>

// Defined in linker script.
extern struct cmdline_option __ld_cmdline_start[];
extern struct cmdline_option __ld_cmdline_end[];

[[__init]]
void boot_cmdline(char* cmdline) {
    struct cmdline_option* opt = __ld_cmdline_start;
    const size_t len = strlen(cmdline);
    const char* cmd = cmdline;

    while (cmd < cmdline + len) {
        // TODO
    }
}

static void cmd_hello(const char* value) {
    kprintf("Hello %s world!\n", value);
}

CMDLINE_OPTION("hello", cmd_hello);
