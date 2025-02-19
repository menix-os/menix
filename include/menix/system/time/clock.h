// Clocks for absolute timekeeping.

#pragma once
#include <menix/common.h>

// Checks a condition with a timeout.
#define clock_timeout_poll(timeout_ns, condition, fail_case) \
	do \
	{ \
		usize check_start = clock_get_elapsed_ns() + (timeout_ns); \
		while (clock_get_elapsed_ns() < check_start) \
		{ \
			if (condition) \
				break; \
			asm_pause(); \
		} \
		if (clock_get_elapsed_ns() >= check_start) \
		{ \
			fail_case \
		} \
	} while (0)

typedef struct
{
	// Name of this clock source.
	const char* name;

	// Returns how many nanoseconds have elapsed since initialization.
	usize (*get_elapsed_ns)(void);

	// Resets the clock's counter.
	void (*reset)(void);
} ClockSource;

// Registers a `source` as the new time
void clock_register(ClockSource* source);

// If a clock source is available, returns the current time elapsed since init.
// If not, returns 0.
usize clock_get_elapsed_ns();

// Updates the base of the counter.
void clock_set_elapsed_ns(usize value);

// Spins for `ns` nanoseconds.
void clock_wait(usize ns);
