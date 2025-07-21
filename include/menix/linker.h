#ifndef _MENIX_LINKER_H
#define _MENIX_LINKER_H

#include <stdint.h>

extern uint8_t _LD_KERNEL_START[];
extern uint8_t _LD_TEXT_START[];
extern uint8_t _LD_TEXT_END[];
extern uint8_t _LD_RODATA_START[];
extern uint8_t _LD_RODATA_END[];
extern uint8_t _LD_DATA_START[];
extern uint8_t _LD_PERCPU_START[];
extern uint8_t _LD_PERCPU_END[];
extern uint8_t _LD_DATA_END[];
extern uint8_t _LD_KERNEL_END[];
extern uint8_t _LD_VDSO_START[];
extern uint8_t _LD_VDSO_END[];

#endif
