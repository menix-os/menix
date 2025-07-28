#ifndef _MENIX_LINKER_H
#define _MENIX_LINKER_H

#include <menix/types.h>

extern u8 __ld_kernel_start[];
extern u8 __ld_text_start[];
extern u8 __ld_text_end[];
extern u8 __ld_rodata_start[];
extern u8 __ld_rodata_end[];
extern u8 __ld_data_start[];
extern u8 __ld_percpu_start[];
extern u8 __ld_percpu_end[];
extern u8 __ld_data_end[];
extern u8 __ld_kernel_end[];
extern u8 __ld_vdso_start[];
extern u8 __ld_vdso_end[];
extern u8 __ld_cmdline_start[];
extern u8 __ld_cmdline_end[];

#endif
