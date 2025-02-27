// Interrupt Descriptor Table management

#pragma once

#include <menix/common.h>

#include <gdt.h>

#define IDT_SIZE			 256
#define IDT_GATE_INT		 0xE
#define IDT_GATE_TRAP		 0xF
#define IDT_TYPE(priv, gate) ((1 << 7) | (((priv) & 0x3) << 0x5) | ((gate) & 0xF))

// IDT Interrupt Descriptor
typedef struct [[gnu::packed]]
{
	u16 base_0_15;
	u16 selector;
#ifdef __x86_64__
	Bits ist:2;
	Bits reserved:6;
#else
	Bits reserved:8;
#endif
	u8 type;
	u16 base_16_31;
#ifdef __x86_64__
	u32 base_32_63;
	u32 reserved2;
#endif
} IdtDesc;

static_assert(sizeof(IdtDesc) == 16);

// IDT Register emulation so it can be accessed from C.
typedef struct [[gnu::packed]]
{
	u16 limit;
	IdtDesc* base;
} IdtRegister;

static_assert(sizeof(IdtRegister) == 10);

typedef struct Context Context;
typedef Context* (*IdtCallbackFn)(usize isr, Context* regs);

// Install the Interrupt Descriptor Table.
void idt_init();

// Sets the gate for one entry in the IDT.
void idt_set(u8 idx, void* offset, u8 type_attr);

// Loads the IDT into its register.
void idt_reload();
