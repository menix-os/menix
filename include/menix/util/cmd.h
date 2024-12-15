// Command line options

#pragma once
#include <menix/common.h>
#include <menix/system/boot.h>

#define CMDLINE_MAX_LENGTH 1024

// Initializes the command line. Don't forget to call `cmd_init`!
void cmd_early_init(const char* data);

// Finalizes command line initialization by copying over the command line data.
void cmd_init();

// Returns a string from the command line matching the given key.
// If not present, returns `fallback`.
// Causes a string allocation.
char* cmd_get_str(const char* key, const char* fallback);

// Returns a number from the command line matching the given key.
// If not present, returns `fallback`.
isize cmd_get_isize(const char* key, isize fallback);

// Returns a number from the command line matching the given key.
// If not present, returns `fallback`.
usize cmd_get_usize(const char* key, usize fallback);
