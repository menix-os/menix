#ifndef _KERNEL_MEM_USER_H
#define _KERNEL_MEM_USER_H

#include <kernel/util/attributes.h>
#include <stddef.h>
#include <stdint.h>

void copy_from_user(uint8_t* dst, const uint8_t __user* src, size_t num);
void copy_to_user(uint8_t __user* dst, const uint8_t* src, size_t num);

#endif
