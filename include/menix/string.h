/*-------------------------------
Kernel C library - String utility
-------------------------------*/

#pragma once

#include <menix/stddef.h>
#include <menix/stdint.h>

void*	memccpy(void* restrict, const void* restrict, int32_t, size_t);
void*	memchr(const void*, int32_t, size_t);
int32_t memcmp(const void*, const void*, size_t);
void*	memcpy(void* restrict, const void* restrict, size_t);
void*	memmove(void*, const void*, size_t);
void*	memset(void*, int32_t, size_t);
char*	stpcpy(char* restrict, const char* restrict);
char*	stpncpy(char* restrict, const char* restrict, size_t);
char*	strcat(char* restrict, const char* restrict);
char*	strchr(const char*, int32_t);
int32_t strcmp(const char*, const char*);
int32_t strcoll(const char*, const char*);
char*	strcpy(char* restrict, const char* restrict);
size_t	strcspn(const char*, const char*);
char*	strdup(const char*);
char*	strerror(int32_t);
int32_t strerror_r(int32_t, char*, size_t);
size_t	strlen(const char*);
char*	strncat(char* restrict, const char* restrict, size_t);
int32_t strncmp(const char*, const char*, size_t);
char*	strncpy(char* restrict, const char* restrict, size_t);
char*	strndup(const char*, size_t);
size_t	strnlen(const char*, size_t);
char*	strpbrk(const char*, const char*);
char*	strrchr(const char*, int32_t);
char*	strsignal(int32_t);
size_t	strspn(const char*, const char*);
char*	strstr(const char*, const char*);
char*	strtok(char* restrict, const char* restrict);
char*	strtok_r(char* restrict, const char* restrict);
char*	strxfrm(char* restrict, const char* restrict, size_t);
