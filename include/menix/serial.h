//? Serial console output

#pragma once

#include <menix/common.h>

void serial_initialize(void);
void serial_putchar(char c);
void serial_write(const char* data, size_t size);
void serial_writestring(const char* data);
