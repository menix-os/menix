#ifndef _MENIX_BOOT_CMDLINE_H
#define _MENIX_BOOT_CMDLINE_H

#include <menix/util/attributes.h>

#define CMDLINE_OPTION(opt_name, opt_func) \
    [[__used, __section(".cmdline")]] \
    static struct cmdline_option __cmline_option_##opt_name = { \
        .name = #opt_name, \
        .func = opt_func, \
    }

struct cmdline_option {
    // The name of this option.
    const char* name;
    // Gets run if this option is present on the command line.
    // If an option is specified as `name=value`, then the `value` is passed as well.
    // Otherwise, it's NULL.
    void (*func)(const char* value);
};

// Parses the command line and invokes all options.
void boot_cmdline(char* cmdline);

#endif
