#include <kernel/arch/sys.h>
#include <kernel/sys/percpu.h>
#include <x86_64/asm.h>
#include <x86_64/defs.h>

void arch_bsp_init() {
    asm_wrmsr(MSR_GS_BASE, (uint64_t)&percpu_bsp);
    asm_wrmsr(MSR_FS_BASE, 0);
    asm_wrmsr(MSR_KERNEL_GS_BASE, 0);
    percpu_write(online, true);
}

[[noreturn]]
void arch_panic() {
    asm volatile("cli; hlt");
    __builtin_unreachable();
}
