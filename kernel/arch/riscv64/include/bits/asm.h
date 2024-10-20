#pragma once

#include <menix/common.h>

#define asm_get_frame_pointer(x)	  asm volatile("mv %0, s0" : "=r"(x))
#define asm_pause()					  asm volatile("pause")
#define asm_get_register(result, reg) asm volatile("mv %0, " #reg : "=r"(result)::"memory")
#define asm_interrupt_disable()		  asm volatile("csrci sstatus, 0x2" ::: "memory")
#define asm_interrupt_enable()		  asm volatile("csrsi sstatus, 0x2" ::: "memory")
