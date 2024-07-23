// x86 specific bits of code

#pragma once

#include <menix/common.h>

/// \brief Contains all accessible x86_64 registers.
typedef struct
{
	uint64_t r15;
	uint64_t r14;
	uint64_t r13;
	uint64_t r12;
	uint64_t r11;
	uint64_t r10;
	uint64_t r9;
	uint64_t r8;
	uint64_t rsi;
	uint64_t rdi;
	uint64_t rbp;
	uint64_t rdx;
	uint64_t rcx;
	uint64_t rbx;
	uint64_t rax;
	uint64_t core;
	uint64_t isr;
	uint64_t error;
	uint64_t rip;
	uint64_t cs;
	uint64_t rflags;
	uint64_t rsp;
	uint64_t ss;
} CpuRegisters;

uint8_t read8(uint16_t port);
uint16_t read16(uint16_t port);
uint32_t read32(uint16_t port);
#ifdef CONFIG_64_bit
uint64_t read64(uint16_t port);
#endif

void write8(uint16_t port, uint8_t value);
void write16(uint16_t port, uint16_t value);
void write32(uint16_t port, uint32_t value);
#ifdef CONFIG_64_bit
void write64(uint16_t port, uint64_t value);
#endif

#define interrupt_disable() asm volatile("cli")
#define interrupt_enable()	asm volatile("sti")

#define PIC1_COMMAND_PORT 0x20
#define PIC1_DATA_PORT	  0x21
#define PIC2_COMMAND_PORT 0xA0
#define PIC2_DATA_PORT	  0xA1
