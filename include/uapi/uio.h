#ifndef MENIX_UAPI_UIO_H
#define MENIX_UAPI_UIO_H

#include <stddef.h>

struct iovec {
    void* base;
    size_t len;
};

#endif
