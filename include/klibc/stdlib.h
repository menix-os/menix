// Kernel C library - stdlib.h

#pragma once

#include <menix/common.h>

void abort();
i32 atoi(char* num, u32 base);
u32 atou(char* num, u32 base);
char* itoa(i32 num, char* str, u32 base);
char* utoa(u32 num, char* str, u32 base);
char* lutoa(u64 num, char* str, u32 base);
