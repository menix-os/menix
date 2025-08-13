#include <kernel/sys/syscalls.h>
#include <menix/status.h>
#include <menix/system.h>
#include <stddef.h>

static menix_status_t do_syscall(size_t num, size_t a0, size_t a1, size_t a2, size_t a3, size_t a4, size_t a5) {
    size_t value;
#if defined(__x86_64__)
    register size_t r3 asm("r10") = a3;
    register size_t r4 asm("r8") = a4;
    register size_t r5 asm("r9") = a5;
    asm volatile("syscall"
                 : "=a"(value)
                 : "a"(num), "D"(a0), "S"(a1), "d"(a2), "r"(r3), "r"(r4), "r"(r5)
                 : "memory", "rcx", "r11");
#elif defined(__aarch64__)
    register size_t rnum asm("x8") = num;
    register size_t value asm("x0");
    register size_t r0 asm("x0") = a0;
    register size_t r1 asm("x1") = a1;
    register size_t r2 asm("x2") = a2;
    register size_t r3 asm("x3") = a3;
    register size_t r4 asm("x4") = a4;
    register size_t r5 asm("x5") = a5;
    asm volatile("svc 0" : "=r"(value) : "r"(rnum), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5) : "memory");
#elif defined(__riscv) && (__riscv_xlen == 64)
    register size_t rnum asm("a7") = num;
    register size_t value asm("a0");
    register size_t r0 asm("a0") = a0;
    register size_t r1 asm("a1") = a1;
    register size_t r2 asm("a2") = a2;
    register size_t r3 asm("a3") = a3;
    register size_t r4 asm("a4") = a4;
    register size_t r5 asm("a5") = a5;
    asm volatile("ecall" : "=r"(value) : "r"(rnum), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5) : "memory");
#elif defined(__loongarch64)
    register size_t rnum asm("a7") = num;
    register size_t value asm("a0");
    register size_t r0 asm("a0") = a0;
    register size_t r1 asm("a1") = a1;
    register size_t r2 asm("a2") = a2;
    register size_t r3 asm("a3") = a3;
    register size_t r4 asm("a4") = a4;
    register size_t r5 asm("a5") = a5;
    asm volatile("syscall 0"
                 : "=r"(value)
                 : "r"(rnum), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5)
                 : "memory");
#else
#error "Unsupported architecture!"
#endif
    return value;
}

void menix_panic(menix_status_t error) {
    do_syscall(SYSCALL_PANIC, error, 0, 0, 0, 0, 0);
    __builtin_unreachable();
}

void menix_log(const char* message, size_t length) {
    do_syscall(SYSCALL_LOG, (size_t)message, length, 0, 0, 0, 0);
}
