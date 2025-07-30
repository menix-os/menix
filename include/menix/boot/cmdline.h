#ifndef _MENIX_BOOT_CMDLINE_H
#define _MENIX_BOOT_CMDLINE_H

struct cmdline_option {
    // The name of this option.
    const char* name;
    // Gets run if this option is present on the command line.
    // If an option is specified as `name=value`, then the `value` is passed as well.
    // Otherwise, it's null.
    void (*func)(const char* value);
};

void cmdline_setup(const char* cmdline);

#endif
