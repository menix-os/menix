#ifndef _MENIX_COMPILER_H
#define _MENIX_COMPILER_H

#if __SIZEOF_INT__ != 4
#error "Compiler is misconfigured!"
#endif

#if __SIZEOF_LONG_LONG__ != 8
#error "Compiler is misconfigured!"
#endif

#endif
