#ifndef _UAPI_MENIX_SYSCALL_H
#define _UAPI_MENIX_SYSCALL_H

#include <uapi/menix/types.h>

struct syscall_result {
	__usize value;
	__usize error;
};

#endif
