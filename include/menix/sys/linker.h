#ifndef _MENIX_SYS_LINKER_H
#define _MENIX_SYS_LINKER_H

#include <stdint.h>

extern uint8_t __ld_kernel_start[];
extern uint8_t __ld_text_start[];
extern uint8_t __ld_text_end[];
extern uint8_t __ld_rodata_start[];
extern uint8_t __ld_rodata_end[];
extern uint8_t __ld_data_start[];
extern uint8_t __ld_percpu_start[];
extern uint8_t __ld_percpu_end[];
extern uint8_t __ld_data_end[];
extern uint8_t __ld_kernel_end[];
extern uint8_t __ld_vdso_start[];
extern uint8_t __ld_vdso_end[];
extern uint8_t __ld_cmdline_start[];
extern uint8_t __ld_cmdline_end[];

#endif
