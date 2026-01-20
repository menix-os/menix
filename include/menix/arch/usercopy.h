#pragma once

#include <menix/compiler.h>
#include <stddef.h>
#include <stdint.h>

struct usercopy_region {
    void (*start_ip)();
    void (*end_ip)();
    void (*fault_ip)();
};

bool arch_usercopy_read(uint8_t* dst, const __user uint8_t* src, size_t len);

bool arch_usercopy_write(__user uint8_t* dst, const uint8_t* src, size_t len);

bool arch_usercopy_strlen(const __user uint8_t* str, size_t max, size_t* len);
