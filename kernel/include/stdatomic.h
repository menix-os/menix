#ifndef MENIX_STDATOMIC_H
#define MENIX_STDATOMIC_H

#include <stdint.h>

#define atomic _Atomic

typedef enum memory_order {
    memory_order_relaxed = __ATOMIC_RELAXED,
    memory_order_consume = __ATOMIC_CONSUME,
    memory_order_acquire = __ATOMIC_ACQUIRE,
    memory_order_release = __ATOMIC_RELEASE,
    memory_order_acq_rel = __ATOMIC_ACQ_REL,
    memory_order_seq_cst = __ATOMIC_SEQ_CST
} memory_order;

#define __ATOMIC_CAST(ptr) _Generic(typeof(ptr), \
    atomic bool*: ((bool*)ptr), \
    atomic uint8_t*: ((uint8_t*)ptr), \
    atomic uint16_t*: ((uint16_t*)ptr), \
    atomic uint32_t*: ((uint32_t*)ptr), \
    atomic uint64_t*: ((uint64_t*)ptr), \
    atomic int8_t*: ((int8_t*)ptr), \
    atomic int16_t*: ((int16_t*)ptr), \
    atomic int32_t*: ((int32_t*)ptr), \
    atomic int64_t*: ((int64_t*)ptr), \
    atomic(void*)*: ((void**)ptr) \
)

#define atomic_store_explicit(ptr, des, ord) __atomic_store_n(__ATOMIC_CAST(ptr), des, ord)
#define atomic_store(ptr, des)               atomic_store_explicit(ptr, des, __ATOMIC_SEQ_CST)

#define atomic_load_explicit(ptr, ord) __atomic_load_n(__ATOMIC_CAST(ptr), ord)
#define atomic_load(ptr)               atomic_load_explicit(ptr, __ATOMIC_SEQ_CST)

#define atomic_exchange_explicit(ptr, des, ord) __atomic_exchange_n(__ATOMIC_CAST(ptr), des, ord)
#define atomic_exchange(ptr, des)               atomic_exchange_explicit(ptr, des, __ATOMIC_SEQ_CST)

#define atomic_fetch_add_explicit(ptr, op, ord) __atomic_fetch_add_n(__ATOMIC_CAST(ptr), op, ord)
#define atomic_fetch_add(ptr, op)               atomic_fetch_add_explicit(ptr, op, __ATOMIC_SEQ_CST)

#define atomic_fetch_sub_explicit(ptr, op, ord) __atomic_fetch_sub_n(__ATOMIC_CAST(ptr), op, ord)
#define atomic_fetch_sub(ptr, op)               atomic_fetch_sub_explicit(ptr, op, __ATOMIC_SEQ_CST)

#define atomic_fetch_or_explicit(ptr, op, ord) __atomic_fetch_or_n(__ATOMIC_CAST(ptr), op, ord)
#define atomic_fetch_or(ptr, op)               atomic_fetch_or_explicit(ptr, op, __ATOMIC_SEQ_CST)

#define atomic_fetch_xor_explicit(ptr, op, ord) __atomic_fetch_xor_n(__ATOMIC_CAST(ptr), op, ord)
#define atomic_fetch_xor(ptr, op)               atomic_fetch_xor_explicit(ptr, op, __ATOMIC_SEQ_CST)

#define atomic_fetch_and_explicit(ptr, op, ord) __atomic_fetch_and_n(__ATOMIC_CAST(ptr), op, ord)
#define atomic_fetch_and(ptr, op)               atomic_fetch_and_explicit(ptr, op, __ATOMIC_SEQ_CST)

#endif
