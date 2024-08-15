// Kernel logging

#pragma once

#include <menix/common.h>

// Expression must be true, or else execution will stop.
#define kassert(expr, msg) \
	if (!(expr)) \
	{ \
		kmesg("Assertion failed: " msg "\nExpression:\n\t" #expr "\n" __FILE__ ":" __PASTE_STR(__LINE__) "\n"); \
		ktrace(); \
	}

typedef struct ATTR(packed) StackFrame
{
	struct StackFrame* prev;	// The inner frame.
	void* return_addr;			// The address this frame returns to.
} StackFrame;

// Print a message to the kernel log.
void kmesg(const char* fmt, ...);

// Print a stack trace to the kernel log.
void ktrace();

// Abort kernel execution.
ATTR(noreturn) void kabort();
