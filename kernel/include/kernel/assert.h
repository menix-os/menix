#pragma once

#include <kernel/compiler.h>
#include <kernel/panic.h>
#include <kernel/print.h>

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
