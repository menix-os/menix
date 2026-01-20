#pragma once

#include <menix/compiler.h>
#include <menix/errno.h>
#include <bits/usercopy.h>
#include <stddef.h>
#include <stdint.h>

struct usercopy_region {
    void (*start_ip)();
    void (*end_ip)();
    void (*fault_ip)();
};

// Copies a block of data from user to kernel memory.
bool usercopy_read(uint8_t* dst, const __user uint8_t* src, size_t len) {
    return arch_usercopy_read(dst, src, len);
}

// Copies a block of data from kernel to user memory.
bool usercopy_write(__user uint8_t* dst, const uint8_t* src, size_t len) {
    return arch_usercopy_write(dst, src, len);
}

// Performs a strlen() on a user string.
bool usercopy_strlen(const __user uint8_t* str, size_t max, size_t* len) {
    return arch_usercopy_strlen(str, max, len);
}
