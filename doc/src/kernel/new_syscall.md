# Adding a new system call

A syscall always has the same synopsis. You should `#include <menix/syscall/syscall.h>`
and use the `SYSCALL_IMPL()` macro in the function head, e.g.:

```c
#include <menix/syscall/syscall.h>

SYSCALL_IMPL(mycall)
{
	return 0;
}
```

## Registering your system call
First, answer this question: Is your syscall architecture dependent?

No? Use `menix/syscall_list.h`. Otherwise add your function to `archctl` instead.
That way all architectures can share the same syscall numbers.

Now, to add your syscall, add a new line following the schema of `SYSCALL(num, name)`,
e.g.:

```c
SYSCALL(42, mycall)
```

> Note that there is no trailing semicolon or comma.

Your syscall should now be reachable from user space.
