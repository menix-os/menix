// A global list of all system calls available.

#include <menix/util/syscall_insert.h>

SYSCALL(0, null)
SYSCALL(1, mmap)

#include <bits/syscall_list.h>
