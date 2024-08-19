// Dynamic array (list) data structure.

#pragma once

#define List(type) \
	struct \
	{ \
		type* items; \
		usize capacity; \
		usize length; \
	}
