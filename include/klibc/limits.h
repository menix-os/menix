// Integer limits

#pragma once

#define INT8_MIN  (-(1U << (8 - 1)))
#define INT16_MIN (-(1U << (16 - 1)))
#define INT32_MIN (-(1U << (32 - 1)))
#if MENIX_BITS >= 64
#define INT64_MIN (-(1U << (64 - 1)))
#endif

#define INT8_MAX  ((1U << (8 - 1)) - 1)
#define INT16_MAX ((1U << (16 - 1)) - 1)
#define INT32_MAX ((1U << (32 - 1)) - 1)
#if MENIX_BITS >= 64
#define INT64_MAX ((1U << (64 - 1)) - 1)
#endif

#define UINT8_MAX  ((1U << 8) - 1)
#define UINT16_MAX ((1U << 16) - 1)
#define UINT32_MAX ((1U << 32) - 1)
#if MENIX_BITS >= 64
#define UINT64_MAX ((1U << 64) - 1)
#endif
