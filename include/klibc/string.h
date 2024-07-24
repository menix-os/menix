// Kernel C library - string.h

#pragma once

#include <menix/common.h>

void* memccpy(void* restrict, const void* restrict, i32, usize);
void* memchr(const void*, i32, usize);
i32 memcmp(const void*, const void*, usize);
void* memcpy(void* restrict, const void* restrict, usize);
void* memmove(void*, const void*, usize);
void* memset(void*, i32, usize);
char* stpcpy(char* restrict, const char* restrict);
char* stpncpy(char* restrict, const char* restrict, usize);
char* strcat(char* restrict, const char* restrict);
char* strchr(const char*, i32);
i32 strcmp(const char*, const char*);
i32 strcoll(const char*, const char*);
char* strcpy(char* restrict, const char* restrict);
usize strcspn(const char*, const char*);
char* strdup(const char*);
char* strerror(i32);
i32 strerror_r(i32, char*, usize);
usize strlen(const char*);
