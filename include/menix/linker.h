#ifndef _MENIX_LINKER_H
#define _MENIX_LINKER_H

#include <menix/types.h>

extern u8 _LD_KERNEL_START[];
extern u8 _LD_TEXT_START[];
extern u8 _LD_TEXT_END[];
extern u8 _LD_RODATA_START[];
extern u8 _LD_RODATA_END[];
extern u8 _LD_DATA_START[];
extern u8 _LD_PERCPU_START[];
extern u8 _LD_PERCPU_END[];
extern u8 _LD_DATA_END[];
extern u8 _LD_KERNEL_END[];
extern u8 _LD_VDSO_START[];
extern u8 _LD_VDSO_END[];

#endif
