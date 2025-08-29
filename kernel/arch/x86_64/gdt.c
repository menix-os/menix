#include <string.h>
#include <x86_64/gdt.h>

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

void gdt_new(struct gdt* gdt) {
    memcpy(gdt, &main_gdt, sizeof(struct gdt));
}
