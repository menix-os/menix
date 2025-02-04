// Console/Terminal IO

#pragma once
#include <menix/common.h>

#define TERMINAL_MAX 8

typedef struct
{
	isize (*read)(void* data, usize length);
	isize (*write)(const void* data, usize length);
} Terminal;

extern Terminal terminal_global;

void terminal_init();

// Writes a string to the active terminal.
void terminal_puts(const char* buf, usize len);
