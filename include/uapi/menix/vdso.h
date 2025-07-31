#ifndef _UAPI_MENIX_VDSO_H
#define _UAPI_MENIX_VDSO_H

#include <uapi/menix/syscall.h>
#include <uapi/menix/types.h>

#define __vdsocall __attribute__((weak))

__vdsocall struct __syscall_result
__menix_vdso_syscall(__usize num, __usize a0, __usize a1, __usize a2, __usize a3, __usize a4, __usize a5);

#endif
