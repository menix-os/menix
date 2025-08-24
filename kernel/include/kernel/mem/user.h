#pragma once

#include <kernel/compiler.h>
#include <stddef.h>
#include <stdint.h>

// Copies a block of data from user to kernel memory.
void user_to_kernel(uint8_t* dst, const uint8_t __user* src, size_t num);

// Copies a block of data from kernel to user memory.
void kernel_to_user(uint8_t __user* dst, const uint8_t* src, size_t num);
