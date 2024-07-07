//? System call interface + prototypes

#pragma once

#include <menix/common.h>

#define SYSCALL_DEFINE0(name)						  size_t name()
#define SYSCALL_DEFINE1(name, a0)					  size_t name(a0)
#define SYSCALL_DEFINE2(name, a0, a1)				  size_t name(a0, a1)
#define SYSCALL_DEFINE3(name, a0, a1, a2)			  size_t name(a0, a1, a2)
#define SYSCALL_DEFINE4(name, a0, a1, a2, a3)		  size_t name(a0, a1, a2, a3)
#define SYSCALL_DEFINE5(name, a0, a1, a2, a3, a4)	  size_t name(a0, a1, a2, a3, a4)
#define SYSCALL_DEFINE6(name, a0, a1, a2, a3, a4, a5) size_t name(a0, a1, a2, a3, a4, a5)
