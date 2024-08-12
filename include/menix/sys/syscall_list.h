// A global list of all system calls available.

#include <menix/sys/syscall_insert.h>

SYSCALL(0, null)
SYSCALL(1, mmap)
SYSCALL(2, exec)

#include <bits/syscall_list.h>
