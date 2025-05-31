#ifndef __MENIX_UAPI_TIME_H
#define __MENIX_UAPI_TIME_H

#include "types.h"

typedef __isize time_t;

struct timespec {
  time_t tv_sec;
  __isize tv_nsec;
};

#endif
