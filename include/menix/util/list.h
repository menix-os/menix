// Dynamic array (list) data structure.

#pragma once
#include <menix/memory/alloc.h>

#include <string.h>

#define List(type) \
	struct \
	{ \
		type* items; \
		usize capacity; \
		usize length; \
	}

// Initializes a new list that can hold `cap` entries.
#define list_new(list, cap) \
	list = (typeof(list)) \
	{ \
		.items = NULL, .capacity = (cap), .length = 0, \
	}

// Frees the memory associated with the `list`.
#define list_free(list) \
	do \
	{ \
		kfree((list)->items); \
		(list)->items = NULL; \
		(list)->capacity = 0; \
		(list)->length = 0; \
	} while (0)

// Pushes a new `item` to the `list`.
#define list_push(list, item) \
	do \
	{ \
		auto __list = (list); \
		/* Default capacity. */ \
		if (__list->capacity == 0) \
			__list->capacity = 16; \
		/* If no container was allocated, do it now.*/ \
		if (__list->items == NULL) \
		{ \
			__list->items = kzalloc(sizeof(typeof(*(__list->items))) * __list->capacity); \
			__list->length = 0; \
		} \
		/* If the new entry would overflow the buffer, double the capacity. */ \
		if (__list->length + 1 > __list->capacity) \
		{ \
			__list->capacity *= 2; \
			__list->items = krealloc(__list->items, sizeof(typeof(*(__list->items))) * __list->capacity); \
		} \
		__list->items[__list->length] = (item); \
		__list->length++; \
	} while (0)

// Removes the element at index `idx` and moves all other members so the list is contiguous again.
#define list_pop(list, idx) \
	do \
	{ \
		auto __list = (list); \
		if ((idx) >= __list->length) \
			break; \
		/* Move all data back by one entry. */ \
		memmove(__list->items + (idx), __list->items + (idx) + 1, \
				sizeof(typeof(*(__list->items))) * (__list->length - (idx))); \
		__list->length--; \
	} while (0)

// Iterates over `list` with iterator variable `var_name`.
#define list_iter(list, var_name) \
	for (typeof(((list)->items)) var_name = (list)->items + 0; (var_name) < (list)->items + (list)->length; var_name++)

// Looks for a `value` in the `list` and returns the index of the first match in `result`. If unsuccessful, returns -1.
#define list_find(list, result, value) \
	({ \
		result = -1; \
		for (usize __i = 0; __i < (list)->length; __i++) \
		{ \
			if ((list)->items[__i] == value) \
			{ \
				result = __i; \
				break; \
			} \
		} \
		true; \
	})
