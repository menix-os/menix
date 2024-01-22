#ifndef _MENIX_TTY_H
#define _MENIX_TTY_H

#include <stddef.h>
#include <menix/stdint.h>

void terminal_initialize(void);
void terminal_putchar(char c);
void terminal_write(const char* data, size_t size);
void terminal_writestring(const char* data);
void terminal_setcolor(uint8_t color);
 
#endif // _MENIX_TTY_H