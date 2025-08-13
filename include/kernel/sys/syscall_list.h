#ifndef SYSCALL
#define SYSCALL(num, name)
#endif

SYSCALL(0, panic)
SYSCALL(1, powerctl)
SYSCALL(2, log)
SYSCALL(3, object_check)
SYSCALL(4, object_close)
SYSCALL(5, object_duplicate)
SYSCALL(6, action_register)
SYSCALL(7, action_unregister)
SYSCALL(8, action_await)
