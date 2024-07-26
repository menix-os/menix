// Interrupt Descriptor Table management

#pragma once

#include <menix/common.h>

#include <gdt.h>

#define IDT_MAX_SIZE		 256
#define IDT_GATE_INT		 0xE
#define IDT_GATE_TRAP		 0xF
#define IDT_TYPE(priv, gate) ((1 << 7) | (((priv) & 0x3) << 5) | ((gate) & 0xF))

//! Here are a few inline assembly macros to create some stubs since we have so many of them.

// Declares an interrupt handler.
#define INT_HANDLER_DECL(num) extern void int_error_handler_##num(void);
#define INT_HANDLER_COMMON(num) \
	".global int_error_handler_" #num "\n" \
	".align 0x10\n" \
	"int_error_handler_" #num ":\n"
// Interrupt handler stub without error code. Calls `fn`.
#define INT_HANDLER(num, fn) \
	INT_HANDLER_DECL(num) \
	asm(INT_HANDLER_COMMON(num) "mov $" #num ", %edi\n" \
								"call " #fn "\n" \
								"iretq\n")
// Interrupt handler stub with error code. Calls `fn`.
#define INT_HANDLER_WITH_CODE(num, fn) \
	INT_HANDLER_DECL(num) \
	asm(INT_HANDLER_COMMON(num) "mov $" #num ", %edi\n" \
								"pop %rsi\n" \
								"call " #fn "\n" \
								"iretq\n")

// IDT Interrupt Descriptor
typedef struct ATTR(packed)
{
	u16 base_0_15;
	u16 selector;
#ifdef CONFIG_64_bit
	Bits ist:2;
	Bits reserved:6;
#else
	bits reserved:8;
#endif
	u8 type;
	u16 base_16_31;
#ifdef CONFIG_64_bit
	u32 base_32_63;
	u32 reserved2;
#endif
} IdtDesc;

// IDT Register emulation so it can be accessed from C.
typedef struct ATTR(packed)
{
	u16 limit;
	IdtDesc* base;
} IdtRegister;

void idt_init();

// Sets the gate for one entry in the IDT.
void idt_set(u8 idx, void* offset, u8 type_attr);

// Loads the IDT into its register.
void idt_reload();
