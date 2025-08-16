#include <kernel/sys/syscalls.h>
#include <menix/system.h>
#include "syscalls.h"

void menix_panic(menix_status_t error) {
    do_syscall1(SYSCALL_PANIC, error);
    __builtin_unreachable();
}

void menix_log(const char* message, size_t length) {
    do_syscall2(SYSCALL_LOG, (size_t)message, length);
}
