#include <menix/percpu.h>
#include <asm.h>
#include <defs.h>
#include <gdt.h>
#include <stddef.h>
#include <string.h>

struct gdt main_gdt = {
    .null = 0,
    .kernel_code32 = 0x00cf9b000000ffff,
    .kernel_data32 = 0x00cf93000000ffff,
    .kernel_code64 = 0x00a09b0000000000,
    .kernel_data64 = 0x0000930000000000,
    .user_code = 0x00cffb000000ffff,
    .user_data = 0x0000f30000000000,
    .user_code64 = 0x0000fb0000000000,
    .tss = {0x0000890000000000, 0},
};

void gdt_init() {
    // Set initial values.
    memcpy(&percpu_get()->arch.gdt, &main_gdt, sizeof(struct gdt));

    struct gdtr gdtr = {
        .limit = sizeof(struct gdt) - 1,
        .base = &percpu_get()->arch.gdt,
    };

    asm volatile("lgdt [%0]" ::"r"(&gdtr));

    // Save the contents of MSR_GS_BASE, as they get cleared by a write to `gs`.
    virt_t gs = asm_rdmsr(MSR_GS_BASE);
    asm volatile(
        "push %0\n"
        "lea rax, [rip + 1f]\n"
        "push rax\n"
        "retfq\n"
        "1:\n"
        "mov ax, %1\n"
        "mov ds, ax\n"
        "mov es, ax\n"
        "mov fs, ax\n"
        "mov gs, ax\n"
        "mov ss, ax\n"
        :
        : "i"(offsetof(struct gdt, kernel_code64)), "i"(offsetof(struct gdt, kernel_data64))
        : "rax", "memory"
    );
    asm_wrmsr(MSR_GS_BASE, gs);
}
