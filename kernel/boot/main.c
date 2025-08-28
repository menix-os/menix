#include <kernel/assert.h>
#include <kernel/boot/cmdline.h>
#include <kernel/boot/init.h>
#include <kernel/mem.h>
#include <kernel/percpu.h>
#include <kernel/print.h>
#include <kernel/sys/console.h>
#include <kernel/sys/irq.h>
#include <config.h>

const char menix_banner[] = "Menix " MENIX_VERSION " (" MENIX_ARCH ", " MENIX_COMPILER_ID ", " MENIX_LINKER_ID ")";

void kernel_early_init() {
    percpu_init_bsp();
    percpu_get()->online = true;
    irq_lock();
}

[[noreturn]]
void kernel_init(struct boot_info* info) {
    cmdline_parse(info->cmdline);
    earlycon_init();
    kprintf("%s\n", menix_banner); // Say hello!

    ASSERT(percpu_get()->online == true, "BSP is not online?");

    // TODO: MM init, scheduler init
    mem_init(info->mem_map, info->num_mem_maps, info->virt_base, info->phys_base, info->hhdm_base);

    irq_unlock();
    // Start scheduling.

    ASSERT(false, "Nothing to do");
}

[[noreturn]]
void kernel_main() {
    while (1) {}
}
