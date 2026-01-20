#include <config.h>
#include <menix/assert.h>
#include <menix/cmdline.h>
#include <menix/compiler.h>
#include <menix/console.h>
#include <menix/init.h>
#include <menix/irq.h>
#include <menix/mem.h>
#include <menix/percpu.h>
#include <menix/print.h>

const char menix_banner[] = "Menix " MENIX_VERSION " (" MENIX_ARCH ", " MENIX_COMPILER_ID ", " MENIX_LINKER_ID ")";

[[__init]]
void kernel_early_init() {
    percpu_bsp_early_init();
    percpu_get()->online = true;
    irq_lock();
}

[[noreturn]]
void kernel_init(struct boot_info* info) {
    cmdline_parse(info->cmdline);
    console_init();
    kprintf("%s\n", menix_banner); // Say hello!
    mem_init(info->mem_map, info->num_mem_maps, info->virt_base, info->phys_base, info->hhdm_base);

    sched_init();
    irq_unlock();

    ASSERT(false, "");

    while (1) {}
}

[[noreturn]]
void kernel_main() {
    while (1) {}

    ASSERT(false, "Nothing to do");
}
