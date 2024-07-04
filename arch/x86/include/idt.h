//? Interrupt Descriptor Table management

#pragma once

#include <menix/common.h>

#include <gdt.h>

#define IDT_MAX_SIZE			   256
#define KERNEL_CODE_SEGMENT_OFFSET GDT_KERNEL_CODE * 8
#define IDT_INTERRUPT_GATE_32	   0x8E
#define PIC1_COMMAND_PORT		   0x20
#define PIC1_DATA_PORT			   0x21
#define PIC2_COMMAND_PORT		   0xA0
#define PIC2_DATA_PORT			   0xA1

/// \brief IDT Interrupt Descriptor
typedef struct ATTR(packed)
{
	uint16_t offset_0_15;
	uint16_t selector;
#ifdef CONFIG_64_bit
	bits ist:2;
	bits reserved:6;
#else
	bits reserved:8;
#endif
	uint8_t	 type_attr;
	uint16_t offset_16_31;
#ifdef CONFIG_64_bit
	uint32_t offset_32_63;
	uint32_t zero;
#endif
} IdtDesc;

/// \brief IDT Register emulation so it can be accessed from C.
typedef struct ATTR(packed)
{
	uint16_t limit;
	IdtDesc* base;
} IdtRegister;

void idt_set();
void idt_fill(IdtDesc* target, void* offset, uint16_t selector, uint8_t type_attr);
void idt_init();
