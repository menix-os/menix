// x86 advanced programmable interrupt controller

#pragma once

#include <menix/common.h>
#include <menix/system/arch.h>

// Disables the legacy PIC.
void pic_disable();

// Sends an End Of Interrupt signal to the APIC.
void apic_send_eoi();

// Sends an inter-processor interrupt to a local APIC.
// `id`: ID of the local APIC.
// `flags`: Flags for the IPI.
void apic_send_ipi(u32 id, u32 flags);

// Redirects an IRQ to an interrupt line.
void apic_redirect_irq(u32 irq, u8 interrupt);

// Initializes the LAPIC
void lapic_init(usize id);

// Reads data from a LAPIC register.
u32 lapic_read(u32 register);

// Writes data to a LAPIC register.
void lapic_write(u32 register, u32 value);

// Returns the ID of the processor-local APIC.
usize lapic_get_id();

// Handles an interrupt triggered by the timer.
Context* timer_handler(usize isr, Context* regs);
