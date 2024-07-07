//? Kernel C library - stdarg.h

#pragma once

#ifndef va_arg

typedef __builtin_va_list va_list;
#define va_start(ap, ...) __builtin_va_start(ap, 0)
#define va_end(ap)		  __builtin_va_end(ap)
#define va_arg(ap, type)  __builtin_va_arg(ap, type)

#endif
