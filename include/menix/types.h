// Commonly used types

#pragma once

#include <stddef.h>
#include <stdint.h>

// Use the processor word size so we can squeeze as many bits as possible into one variable.
typedef size_t bits;

// Memory mapped IO address.
typedef void* mmio;
