#include <menix/system/interrupts.h>

#include "menix/util/log.h"

void irq_generic_handler(Irq irq)
{
	print_log("irq_generic_handler, %zu", irq);
}
