#include <kernel/boot/cmdline.h>
#include <kernel/boot/init.h>
#include <kernel/sys/console.h>
#include <kernel/sys/print.h>
#include <config.h>

const char menix_banner[] = "Menix " MENIX_VERSION " (" MENIX_COMPILER_ID ", " MENIX_LINKER_ID ")";

void kernel_early_init() {}

[[noreturn]]
void kernel_init(struct boot_info* info) {
    cmdline_parse(info->cmdline);
    earlycon_init();
    kprintf("%s\n", menix_banner);

    while (1) {}
}

[[noreturn]]
void kernel_main(size_t, size_t) {
    while (1) {}
}
