#ifndef _UAPI_MENIX_VDSO_H
#define _UAPI_MENIX_VDSO_H

#include <uapi/menix/types.h>
#include <uapi/menix/syscall.h>

#define __vdsocall __attribute__((weak))

struct __syscall_result __vdsocall
__menix_vdso_syscall(__usize num, __usize a0, __usize a1, __usize a2, __usize a3, __usize a4, __usize a5);

#endif
