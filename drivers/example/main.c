#include <menix/sys/kprintf.h>
#include <menix/sys/module.h>

static errno_t example_init() {
#ifdef __MODULE__
    pr_log("Hello, module world!\n");
#else
    pr_log("Hello, builtin world!\n");
#endif
    return 0;
}

MODULE = {
    .name = "example",
    .init = example_init,
};
