#pragma once

#include <menix/compiler.h>

[[__format(printf, 1, 2)]]
void kprintf(const char* message, ...);
