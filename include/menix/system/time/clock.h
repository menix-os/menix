// Clocks for absolute timekeeping.

#pragma once
#include <menix/common.h>

// Checks a condition with a timeout.
#define clock_timeout_poll(timeout_ns, condition, fail_case) \
	do \
	{ \
		usize check_start = clock_get_elapsed() + (timeout_ns); \
		while (clock_get_elapsed() < check_start) \
		{ \
			if (condition) \
				break; \
			asm_pause(); \
		} \
		if (clock_get_elapsed() >= check_start) \
		{ \
			fail_case \
		} \
	} while (0)

typedef struct
{
	// Name of this clock source.
	const char* name;

	// Returns how many nanoseconds have elapsed since initialization.
	usize (*get_elapsed_ns)();
} ClockSource;

// Registers a `source` as the new time
void clock_register(ClockSource* source);

// If a clock source is available, returns the current time elapsed since init.
// If not, returns 0.
usize clock_get_elapsed();

// Waits `ns` nanoseconds.
void clock_wait(usize ns);
