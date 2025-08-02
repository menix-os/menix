#ifndef _MENIX_BOOT_CMDLINE_H
#define _MENIX_BOOT_CMDLINE_H

#include <menix/util/attributes.h>
#include <menix/util/common.h>

#define CMDLINE_OPTION(opt_name, opt_func) \
    [[__used, __section(".cmdline")]] \
    static const struct cmdline_option CONCAT(__cmline_option_, __COUNTER__) = { \
        .name = opt_name, \
        .func = opt_func, \
    }

struct cmdline_option {
    // The name of this option.
    const char* name;
    // Gets run if this option is present on the command line.
    // If an option is specified as `name=value`, then the `value` is passed as well.
    // Otherwise, it's NULL.
    void* func;
};

// Parses the command line and invokes all options.
void boot_cmdline(char* cmdline);

#endif
