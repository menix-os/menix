/*---------------------------------
Interrupt handler for double fault.
---------------------------------*/

#include <menix/stdio.h>
#include <idt.h>

MENIX_ATTR(noreturn)
void interrupt_error(void)
{
	printf("\nunhandled kernel error!");

	// Stop the kernel.
	__asm__ volatile (
		"cli\n"
		"hlt"
	);
	while (1) {}
}
