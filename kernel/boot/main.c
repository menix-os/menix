#include <kernel/assert.h>
#include <kernel/cmdline.h>
#include <kernel/compiler.h>
#include <kernel/console.h>
#include <kernel/init.h>
#include <kernel/irq.h>
#include <kernel/mem.h>
#include <kernel/percpu.h>
#include <kernel/print.h>
#include <config.h>

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
    earlycon_init();
    kprintf("%s\n", menix_banner); // Say hello!
    mem_init(info->mem_map, info->num_mem_maps, info->virt_base, info->phys_base, info->hhdm_base);

    sched_init();
    irq_unlock();

    while (1) {}
}

[[noreturn]]
void kernel_main() {
    while (1) {}

    ASSERT(false, "Nothing to do");
}
