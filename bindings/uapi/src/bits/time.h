#ifndef __MENIX_UAPI_TIME_H
#define __MENIX_UAPI_TIME_H

typedef long time_t;

struct timespec {
  time_t tv_sec;
  long tv_nsec;
};

#endif
