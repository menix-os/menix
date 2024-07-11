//? Snippet for pasting syscalls.

// SYSCALL_TABLE_INSERT is defined in the global system call table. This way we only need to declare our syscalls once.
#ifdef SYSCALL_TABLE_INSERT
#define SYSCALL(num, name) [num] = syscall_##name,
#else
#define SYSCALL(num, name) size_t syscall_##name(size_t a0, size_t a1, size_t a2, size_t a3, size_t a4, size_t a5);
#endif
