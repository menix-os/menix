# Adding a new system call

A syscall always has the same synopsis. You should `#include <menix/syscall.h>`
and use the `SYSCALL_IMPL()` macro in the function head, e.g.:
```c
#include <menix/syscall.h>

SYSCALL_IMPL(mycall)
{
	return 0;
}
```

## Registering your system call
First, answer this question: Is your syscall architecture dependent?

No? Use `menix/syscall_list.h`. Otherwise use `bits/syscall_list.h`
(aka `/kernel/arch/<arch>/include/bits/syscall_list.h`).
That way all architecture dependent syscalls get inserted _after_ the common ones.

Now, to add your syscall, add a new line following the schema of `SYSCALL(num, name)`,
e.g.:
```c
SYSCALL(42, mycall)
```
> Note that there is no trailing semicolon or comma.

Your system call should now be visible to the entirety of the kernel.
