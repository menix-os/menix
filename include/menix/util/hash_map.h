// "Generic" hash map data structure.

#pragma once
#include <menix/common.h>
#include <menix/memory/alloc.h>

// Maximum length of key data.
#define HASHMAP_KEY_LEN 256

// Defines a hash map with a string as key and `type` as value type.
#define HashMap(type) \
	struct \
	{ \
		struct \
		{ \
			struct \
			{ \
				type item; \
				u8 key_data[HASHMAP_KEY_LEN]; \
				usize key_len; \
			}* items; \
			usize capacity; \
			usize count; \
		}* buckets; \
		usize capacity; \
	}

static inline u32 hash(const void* data, usize length)
{
	const u8* data_u8 = data;
	u32 hash = 0;

	for (usize i = 0; i < length; i++)
	{
		u32 c = data_u8[i];
		hash = c + (hash << 6) + (hash << 16) - hash;
	}

	return hash;
}

// Initializes a hashmap `map` with a capacity of `cap` elements.
#define hashmap_init(map, cap) \
	map = (typeof(map)) \
	{ \
		.capacity = cap, .buckets = NULL, \
	}

// Inserts `value` with a connected `key` of `key_length` length into `map`.
#define hashmap_insert(map, key, key_length, value) \
	do \
	{ \
		/* Copy macro values over. */ \
		auto __key = key; \
		auto __key_len = key_length; \
		auto __map = map; \
		/* Allocate buckets */ \
		if (__map->buckets == NULL) \
		{ \
			__map->buckets = kzalloc(__map->capacity * sizeof(*(__map->buckets))); \
		} \
		usize __hash = hash(__key, __key_len); \
		usize __index = __hash % __map->capacity; \
		auto __bucket = &__map->buckets[__index]; \
		/* Allocate items for current bucket. */ \
		if (__bucket->capacity == 0) \
		{ \
			__bucket->capacity = 16; \
			__bucket->items = kzalloc(__bucket->capacity * sizeof(*__bucket->items)); \
		} \
		if (__bucket->count == __bucket->capacity) \
		{ \
			__bucket->capacity *= 2; \
			__bucket->items = krealloc(__bucket->items, __bucket->capacity * sizeof(*__bucket->items)); \
		} \
		auto __item = &__bucket->items[__bucket->count]; \
		memcpy(&__item->key_data[0], __key, __key_len); \
		__item->key_len = __key_len; \
		__item->item = (value); \
		__bucket->count++; \
	} while (0)
