#pragma once

#include <kernel/compiler.h>

[[__format(printf, 1, 2)]]
void kprintf(const char* message, ...);
