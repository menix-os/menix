#ifndef _KERNEL_UTIL_ASSERT_H
#define _KERNEL_UTIL_ASSERT_H

#include <kernel/sys/panic.h>
#include <kernel/sys/print.h>
#include <kernel/util/attributes.h>

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
