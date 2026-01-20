#pragma once

#include <menix/common.h>
#include <menix/compiler.h>
#include <menix/types.h>
#include <stddef.h>
#include <stdint.h>

#define __init                  __used, __section(".init.text"), __cold
#define __initdata              __used, __section(".init.data")
#define __initdata_sorted(name) __used, __section(".init.data." name)

struct boot_file {
    uint8_t* data;
    size_t length;
    const char* path;
};

struct boot_info {
    char* cmdline;
    struct phys_mem* mem_map;
    size_t num_mem_maps;
    phys_t phys_base;
    virt_t virt_base;
    virt_t hhdm_base;
    struct boot_file* files;
    size_t num_files;
};

extern uint8_t __ld_stack_top[];
extern uint8_t __ld_stack_bottom[];

// Entry point for the kernel after arch-specific setup has finished.
void kernel_entry();

// Initializes the early parts of the kernel.
void kernel_early_init();

// Initializes the kernel.
[[noreturn]]
void kernel_init(struct boot_info* info);

// Main thread.
[[noreturn]]
void kernel_main();
