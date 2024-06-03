#pragma once

#include <stdint.h>

typedef struct tss
{
    uint8_t bytes[0x6C];
} tss_t;

static tss_t task_state;
