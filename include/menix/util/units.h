// Macros for SI units and other useful conversions.

#pragma once

#define KiB (usize)(1024ULL)
#define MiB (usize)(1024ULL * 1024ULL)
#define GiB (usize)(1024ULL * 1024ULL * 1024ULL)
#define TiB (usize)(1024ULL * 1024ULL * 1024ULL * 1024ULL)

#define SECONDS_TO_NANO(seconds)  (usize)((seconds) * 1000000000ULL)
#define SECONDS_TO_MILLI(seconds) (usize)((seconds) * 1000000ULL)
