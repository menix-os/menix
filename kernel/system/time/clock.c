// Clock registration.

#include <menix/common.h>
#include <menix/system/arch.h>
#include <menix/system/time/clock.h>
#include <menix/util/log.h>

static ClockSource* current_source = NULL;
static usize clock_base_ns = 0;	   // Base counter in nanoseconds.

void clock_register(ClockSource* source)
{
	if (source == NULL || source->get_elapsed_ns == NULL || source->reset == NULL)
		return;

	print_log("clock: Switching to new source \"%s\"\n", source->name);

	// Move the current value over to the new counter.
	const usize timer = clock_get_elapsed_ns();
	current_source = source;
	clock_set_elapsed_ns(timer);
}

usize clock_get_elapsed_ns()
{
	if (current_source != NULL)
		return current_source->get_elapsed_ns();

	return 0;
}

void clock_set_elapsed_ns(usize value)
{
	clock_base_ns = value;
	if (current_source != NULL)
		current_source->reset();
}

void clock_wait(usize ns)
{
	if (current_source == NULL)
	{
		print_warn("clock: Attempted to wait %zu nanoseconds, "
				   "but this would hang indefinitely since no clock source is available.\n",
				   ns);
		return;
	}

	usize time = clock_get_elapsed_ns() + ns;
	while (time > clock_get_elapsed_ns())
	{
		asm_pause();
	}
}
