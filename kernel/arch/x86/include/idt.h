/*-----------------------------------
Interrupt Descriptor Table management
-----------------------------------*/

#pragma once

#include <menix/common.h>
#include <menix/stdint.h>

#define IDT_MAX_SIZE 256
#define KERNEL_CODE_SEGMENT_OFFSET 0x8
#define IDT_INTERRUPT_GATE_32 0x8E
#define PIC1_COMMAND_PORT 0x20
#define PIC1_DATA_PORT 0x21
#define PIC2_COMMAND_PORT 0xA0
#define PIC2_DATA_PORT 0xA1

extern void keyboard_handler(void);
extern void error_handler(void);
extern char io_in(uint16_t port);
extern void io_out(uint16_t port, uint8_t data);
extern void idt_set(uint16_t limit, uint32_t base);
extern void enable_interrupts(void);
void interrupt_keyboard(void);
void interrupt_error(void);

typedef struct
MENIX_ATTR(packed)
{
    uint16_t offset_low;
    uint16_t selector;
    uint8_t zero;
    uint8_t type_attr;
    uint16_t offset_high;
} IdtEntry;

MENIX_ATTR(aligned(0x10))
static IdtEntry idt_table[IDT_MAX_SIZE];

void idt_fill(uint8_t idx, uint32_t offset, uint16_t selector, uint8_t type_attr);
void idt_init();
