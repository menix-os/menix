#ifndef _MENIX_X86_64_GDT_H
#define _MENIX_X86_64_GDT_H

#include <menix/util/attributes.h>
#include <stdint.h>
#include <tss.h>

struct gdt {
    uint64_t null;
    uint64_t kernel_code32;
    uint64_t kernel_data32;
    uint64_t kernel_code64;
    uint64_t kernel_data64;
    uint64_t user_code;
    uint64_t user_data;
    uint64_t user_code64;
    uint64_t tss[2];
};

struct [[__packed]] gdtr {
    uint16_t limit;
    struct gdt* gdt;
};

// Initializes a GDT with initial values.
void gdt_new(struct gdt* gdt);
void gdt_set_tss(struct gdt* gdt, struct tss* tss);

#endif
