#include <menix/cmdline.h>
#include <menix/init.h>
#include <stddef.h>
#include <string.h>

// Defined in linker script.
extern struct cmdline_option __ld_cmdline_start[];
extern struct cmdline_option __ld_cmdline_end[];

static char cmdline_buf[CMDLINE_MAX];

[[__init]]
void cmdline_parse(const char* cmdline) {
    struct cmdline_option* opt = __ld_cmdline_start;
    const size_t len = strnlen(cmdline, CMDLINE_MAX - 1);
    size_t idx = 0;
    memcpy(cmdline_buf, cmdline, len);

    while (1) {
        char* name = nullptr;
        char* value = nullptr;

        // Skip all leading spaces.
        while (idx < len && cmdline_buf[idx] == ' ')
            idx++;
        if (idx >= len)
            break;
        size_t name_idx = idx;
        name = cmdline_buf + name_idx;

        // Find the next equal sign or space.
        while (idx < len && cmdline_buf[idx] != '=' && cmdline_buf[idx] != ' ')
            idx++;
        if (idx > len)
            break;

        // Check if the option has a value (=foo).
        char seperator = cmdline_buf[idx];
        cmdline_buf[idx++] = 0;
        if (seperator == '=') {
            // Check if we need to escape the value.
            char check;
            if (cmdline_buf[idx] == '"') {
                check = '"';
                cmdline_buf[idx++] = 0;
            } else {
                check = ' ';
            }

            value = cmdline_buf + idx;

            // Skip the value.
            while (idx < len && cmdline_buf[idx] != check)
                idx++;
            if (idx > len)
                break;
            cmdline_buf[idx++] = 0;
        }

        // Find the corresponding option.
        while (opt < __ld_cmdline_end) {
            if (!strcmp(opt->name, name))
                opt->func(value);
            opt++;
        }

        if (idx >= len)
            break;
    }
}
