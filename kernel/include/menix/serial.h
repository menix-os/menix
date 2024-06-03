/*-------------------
Serial console output
-------------------*/

#pragma once

#include <menix/stddef.h>
#include <menix/stdint.h>
#include <menix/common.h>
#include <menix/config.h>

void serial_initialize(void);
void serial_putchar(char c);
void serial_write(const char* data, size_t size);
void serial_writestring(const char* data);
