// Kernel C library - stdlib.h

#pragma once

#include <menix/common.h>

void abort();

i8 atoi8(char* num, u32 base);
i16 atoi16(char* num, u32 base);
i32 atoi32(char* num, u32 base);
i64 atoi64(char* num, u32 base);

u8 atou8(char* num, u32 base);
u16 atou16(char* num, u32 base);
u32 atou32(char* num, u32 base);
u64 atou64(char* num, u32 base);

char* i8toa(i8 num, char* str, u32 base);
char* i16toa(i16 num, char* str, u32 base);
char* i32toa(i32 num, char* str, u32 base);
char* i64toa(i64 num, char* str, u32 base);

char* u8toa(u8 num, char* str, u32 base);
char* u16toa(u16 num, char* str, u32 base);
char* u32toa(u32 num, char* str, u32 base);
char* u64toa(u64 num, char* str, u32 base);
