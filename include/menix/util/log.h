// Kernel logging

#pragma once

#include <menix/common.h>
#include <menix/fs/handle.h>

// Expression must be true, or else execution will stop.
#if !defined(NDEBUG) || CONFIG_force_asserts
#define kassert(expr, msg, ...) \
	do \
	{ \
		if (!(expr)) \
		{ \
			kmesg("Assertion failed!\n" \
				  "[Location]\t%s (%s:%zu)\n" \
				  "[Expression]\t%s\n" \
				  "[Message]\t" msg "\n", \
				  __FUNCTION__, __FILE__, __LINE__, #expr, ##__VA_ARGS__); \
			ktrace(NULL); \
			kabort(); \
		} \
	} while (0)
#else
#define kassert(expr, msg, ...) \
	do \
	{ \
		(void)(expr); \
	} while (0)
#endif

typedef struct ATTR(packed) StackFrame
{
	struct StackFrame* prev;	// The inner frame.
	void* return_addr;			// The address this frame returns to.
} StackFrame;

void kmesg_set_output(Handle* handle);

// Print a message to the kernel log.
void kmesg(const char* fmt, ...);

typedef struct Context Context;

// Print a stack trace to the kernel log.
void ktrace(Context* regs);

// Abort kernel execution.
ATTR(noreturn) void kabort();
