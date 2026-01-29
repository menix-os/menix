#pragma once

#include <kernel/mem.h>
#include <stddef.h>
#include <string.h>

#define VEC(type) \
    struct { \
        type* items; \
        size_t capacity; \
        size_t length; \
    }

// Initializes a vector that can hold `cap` entries.
#define VEC_INIT(vec, cap) \
    do { \
        *(vec) = (typeof(*(vec))){ \
            .items = nullptr, \
            .capacity = (cap), \
            .length = 0, \
        }; \
    } while (0)

// Frees the memory associated with the `vec`.
#define VEC_FREE(vec) \
    do { \
        kfree((vec)->items); \
        (vec)->items = NULL; \
        (vec)->capacity = 0; \
        (vec)->length = 0; \
    } while (0)

// Pushes a new `item` to the `vec`.
#define VEC_PUSH(vec, item) \
    do { \
        auto __vec = (vec); \
        /* Default capacity. */ \
        if (__vec->capacity == 0) \
            __vec->capacity = 16; \
        /* If no container was allocated, do it now.*/ \
        if (__vec->items == nullptr) { \
            __vec->items = kmalloc(sizeof(typeof(*(__vec->items))) * __vec->capacity, KMF_ZEROED); \
            __vec->length = 0; \
        } \
        /* If the new entry would overflow the buffer, double the capacity. */ \
        if (__vec->length + 1 > __vec->capacity) { \
            __vec->capacity *= 2; \
            __vec->items = krealloc(__vec->items, sizeof(typeof(*(__vec->items))) * __vec->capacity); \
        } \
        __vec->items[__vec->length] = (item); \
        __vec->length++; \
    } while (0)

// Removes the element at index `idx` and moves all other members so the vec is contiguous again.
#define VEC_POP(vec, idx) \
    do { \
        auto __vec = (vec); \
        if ((idx) >= __vec->length) \
            break; \
        /* Move all data back by one entry. */ \
        memmove( \
            __vec->items + (idx), \
            __vec->items + (idx) + 1, \
            sizeof(typeof(*(__vec->items))) * (__vec->length - (idx)) \
        ); \
        __vec->length--; \
    } while (0)

// Iterates over `vec` with iterator variable `var_name`.
#define VEC_ITER(vec, var_name) \
    for (typeof(((vec)->items)) var_name = (vec)->items + 0; (var_name) < (vec)->items + (vec)->length; var_name++)

// Looks for a `value` in the `vec` and returns the index of the first match in `result`.
// If unsuccessful, returns -1.
#define VEC_FIND(vec, result, value) \
    do { \
        result = -1; \
        for (usize __i = 0; __i < (vec)->length; __i++) { \
            if ((vec)->items[__i] == value) { \
                result = __i; \
                break; \
            } \
        } \
        true; \
    } while (0)
