#include <kernel/arch/sys.h>
#include <kernel/boot/cmdline.h>
#include <kernel/boot/init.h>
#include <kernel/sys/assert.h>
#include <kernel/sys/console.h>
#include <kernel/sys/percpu.h>
#include <kernel/sys/print.h>
#include <config.h>

const char menix_banner[] = "Menix " MENIX_VERSION " (" MENIX_COMPILER_ID ", " MENIX_LINKER_ID ")";

void kernel_early_init() {
    arch_bsp_init();
}

[[noreturn]]
void kernel_init(struct boot_info* info) {
    cmdline_parse(info->cmdline);
    earlycon_init();
    kprintf("%s\n", menix_banner); // Say hello!

    ASSERT(percpu_read(online) == true, "BSP is not online?");

    // TODO: MM init, scheduler init

    ASSERT(false, "Nothing to do");
}

[[noreturn]]
void kernel_main() {
    while (1) {}
}
