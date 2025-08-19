#ifndef _KERNEL_ASSERT_H
#define _KERNEL_ASSERT_H

#include <kernel/compiler.h>
#include <kernel/panic.h>
#include <kernel/print.h>

#define ASSERT(expr, msg, ...) \
    ({ \
        if (__unlikely(!(expr))) { \
            kprintf( \
                "Environment is unsound! Assertion \"%s\" failed!\n" \
                "In function \"%s\" (%s:%u):\n" msg "\n", \
                #expr, \
                __FUNCTION__, \
                __FILE__, \
                __LINE__, \
                ##__VA_ARGS__ \
            ); \
            panic(); \
        } \
    })

#endif
