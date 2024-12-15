// Interrupt Request routing

#include <menix/system/interrupts.h>
#include <menix/util/log.h>

// Registers a new IRQ handler. Automatically selects optimal IRQ placement.
bool irq_register_handler(IrqFn handler, void* data)
{
	todo();
	return true;
}
