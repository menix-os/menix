// Kernel logging

#pragma once

#include <menix/common.h>
#include <menix/fs/handle.h>

// Expression must be true, or else execution will stop.
#define kassert(expr, msg, ...) \
	if (!(expr)) \
	{ \
		kmesg("Assertion failed:\n\t" msg "\nExpression:\n    " #expr "\n" __FILE__ ":" __PASTE_STR(__LINE__) "\n", \
			  ##__VA_ARGS__); \
		ktrace(); \
		kabort(); \
	}

typedef struct ATTR(packed) StackFrame
{
	struct StackFrame* prev;	// The inner frame.
	void* return_addr;			// The address this frame returns to.
} StackFrame;

void kmesg_set_output(Handle* handle);

// Print a message to the kernel log.
void kmesg(const char* fmt, ...);

// Print a stack trace to the kernel log.
void ktrace();

// Abort kernel execution.
ATTR(noreturn) void kabort();
