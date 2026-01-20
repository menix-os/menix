#pragma once

#include <menix/compiler.h>
#include <menix/panic.h>
#include <menix/print.h>

#define ASSERT(expr, msg, ...) \
    ({ \
        if (__unlikely(!(expr))) { \
            kprintf( \
                "\e[31mKernel panic - Environment is unsound!\n" \
                "In function \"%s\" (%s:%u):\n" \
                "Assertion \"%s\" failed! " msg "\n", \
                __FUNCTION__, \
                __FILE__, \
                __LINE__, \
                #expr, \
                ##__VA_ARGS__ \
            ); \
            panic(); \
        } \
    })
