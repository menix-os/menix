#pragma once

#include <menix/common.h>

#define asm_pause()					  asm volatile("pause")
#define asm_set_register(value, reg)  asm volatile("mv " #reg ", %0" ::"r"(value) : "memory")
#define asm_get_register(result, reg) asm volatile("mv %0, " #reg : "=r"(result)::"memory")
#define asm_interrupt_disable()		  asm volatile("csrci sstatus, 0x2" ::: "memory")
#define asm_interrupt_enable()		  asm volatile("csrsi sstatus, 0x2" ::: "memory")
#define asm_halt()					  asm volatile("wfi")
#define asm_read_csr(csr, result)	  asm volatile("csrr " #csr ", %0" ::"r"(value))
#define asm_write_csr(csr, value)	  asm volatile("csrw " #csr ", %0" ::"r"(value))
