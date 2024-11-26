// Clock registration.

#include <menix/common.h>
#include <menix/system/arch.h>
#include <menix/system/time/clock.h>

static ClockSource* current_source = NULL;

void clock_register(ClockSource* source)
{
	if (source == NULL || source->get_elapsed_ns == NULL)
		return;

	current_source = source;
}

usize clock_get_elapsed()
{
	if (current_source != NULL)
		return current_source->get_elapsed_ns();

	return 0;
}

void clock_wait(usize ns)
{
	usize now = clock_get_elapsed() + ns;
	while (now > clock_get_elapsed())
	{
		asm_pause();
	}
}
