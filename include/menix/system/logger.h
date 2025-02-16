// Kernel logging output

#pragma once
#include <menix/common.h>

typedef isize (*LoggerWriteFn)(const void* data, usize length);

// Registers a new logger callback.
void logger_register(const char* name, LoggerWriteFn callback);

// Writes a string to all loggers.
void logger_write(const char* buf, usize len);
