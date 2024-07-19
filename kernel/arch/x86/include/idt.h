//? Interrupt Descriptor Table management

#pragma once

#include <menix/common.h>

#include <gdt.h>

#define IDT_MAX_SIZE		 256
#define IDT_GATE_INT		 0xE
#define IDT_GATE_TRAP		 0xF
#define IDT_TYPE(priv, gate) ((1 << 7) | (((priv) & 0x3) << 5) | ((gate) & 0xF))

/// \brief IDT Interrupt Descriptor
typedef struct ATTR(packed)
{
	uint16_t base_0_15;
	uint16_t selector;
#ifdef CONFIG_64_bit
	bits ist:2;
	bits reserved:6;
#else
	bits reserved:8;
#endif
	uint8_t	 type;
	uint16_t base_16_31;
#ifdef CONFIG_64_bit
	uint32_t base_32_63;
	uint32_t reserved2;
#endif
} IdtDesc;

/// \brief IDT Register emulation so it can be accessed from C.
typedef struct ATTR(packed)
{
	uint16_t limit;
	IdtDesc* base;
} IdtRegister;

void idt_init();

/// \brief Sets the gate for one entry in the IDT.
void idt_set(uint8_t idx, void* offset, uint8_t type_attr);

/// \brief Loads the IDT into its register.
void idt_reload();
