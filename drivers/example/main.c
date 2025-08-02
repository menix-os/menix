#include <menix/sys/kprintf.h>
#include <menix/sys/module.h>

static errno_t example_init() {
    pr_log("Hello, module world!\n");
    return 0;
}

MODULE = {
    .name = "example",
    .init = example_init,
};
