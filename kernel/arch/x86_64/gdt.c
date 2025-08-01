#include <gdt.h>

struct gdt main_gdt = {
    .null = 0x0000000000000000,
    .kernel_code32 = 0x00cf9b000000ffff,
    .kernel_data32 = 0x00cf93000000ffff,
    .kernel_code64 = 0x00a09b0000000000,
    .kernel_data64 = 0x0000930000000000,
    .user_code = 0x00cffb000000ffff,
    .user_data = 0x0000f30000000000,
    .user_code64 = 0x0000fb0000000000,
    .tss = {0, 0x0000810000000000},
};
