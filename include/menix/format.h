//? Graphics format encoders and helpers

#pragma once

#include <menix/common.h>

#define ENCODE16(a, b)		 ((a) << 8) | (b)
#define ENCODE32(a, b, c, d) ((a) << 24) | ((b) << 16) | ((c) << 8) | (d)
#define ENCODE64(a, b, c, d, e, f, g, h) \
	((a) << 56) | ((b) << 48) | ((c) << 40) | ((d) << 32) | ((e) << 24) | ((f) << 16) | ((g) << 8) | (h)
