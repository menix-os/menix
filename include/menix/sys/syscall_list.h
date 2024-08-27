// A global list of all system calls available.

#include <menix/sys/syscall_insert.h>

SYSCALL(0, exit)
SYSCALL(1, execve)

#include <bits/syscall_list.h>
