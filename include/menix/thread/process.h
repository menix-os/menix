// Process management.

#pragma once

#include <menix/common.h>

typedef usize ProcessId;

typedef struct
{
	ProcessId id;
} Process;

typedef struct
{
	Process* process;	 // The underlying process.
} Thread;

// Creates a new process of an executable pointed to by `path`.
Process* process_create(const char* path);
