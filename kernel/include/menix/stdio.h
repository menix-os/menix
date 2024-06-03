/*------------------------
Kernel C library - stdio.h
------------------------*/

#pragma once

#include <menix/common.h>
#include <menix/stdint.h>

#define EOF (-1)

int32_t printf(const char* restrict, ...);
int32_t putchar(int32_t);
int32_t puts(const char*);
