// Clocks for absolute timekeeping.

#pragma once
#include <menix/common.h>

typedef struct
{
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
