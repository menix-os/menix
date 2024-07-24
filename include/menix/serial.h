// Serial console output

#pragma once

#include <menix/common.h>

#define ANSI_BLACK	 0
#define ANSI_RED	 1
#define ANSI_GREEN	 2
#define ANSI_YELLOW	 3
#define ANSI_BLUE	 4
#define ANSI_MAGENTA 5
#define ANSI_CYAN	 6
#define ANSI_WHITE	 7

#define ANSI_RESET			  "\x1B[0m"
#define ANSI_COLOR(fg)		  "\x1B[3" __PASTE_STR(fg) "m"
#define ANSI_COLOR_BG(fg, bg) "\x1B[3" __PASTE_STR(fg) ";4" __PASTE_STR(bg) "m"

void serial_initialize(void);
void serial_putchar(char c);
void serial_write(const char* data, usize size);
