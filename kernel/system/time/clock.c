// Clock registration.

#include <menix/common.h>
#include <menix/system/arch.h>
#include <menix/system/time/clock.h>
#include <menix/util/log.h>

static ClockSource* current_source = NULL;

void clock_register(ClockSource* source)
{
	if (source == NULL || source->get_elapsed_ns == NULL)
		return;

	print_log("clock: switching to new source \"%s\"\n", source->name);
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
	usize time = clock_get_elapsed() + ns;
	while (time > clock_get_elapsed())
	{
		asm_pause();
	}
}
