#include <uapi/menix/types.h>
#include <uapi/menix/syscall.h>
#include <uapi/menix/vdso.h>

struct __syscall_result
__menix_vdso_syscall(__usize num, __usize a0, __usize a1, __usize a2, __usize a3, __usize a4, __usize a5) {
    struct __syscall_result r;
#if defined(__x86_64__)
    register __usize r3 asm("r10") = a3;
    register __usize r4 asm("r8") = a4;
    register __usize r5 asm("r9") = a5;

    asm volatile("syscall"
                 : "=a"(r.value), "=d"(r.error)
                 : "a"(num), "D"(a0), "S"(a1), "d"(a2), "r"(r3), "r"(r4), "r"(r5)
                 : "memory", "rcx", "r11");
#elif defined(__aarch64__)
    register __usize snum asm("x8") = num;
    register __usize value asm("x0");
    register __usize error asm("x1");
    register __usize r0 asm("x0") = a0;
    register __usize r1 asm("x1") = a1;
    register __usize r2 asm("x2") = a2;
    register __usize r3 asm("x3") = a3;
    register __usize r4 asm("x4") = a4;
    register __usize r5 asm("x5") = a5;
    asm volatile("svc 0"
                 : "=r"(value), "=r"(error)
                 : "r"(snum), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5)
                 : "memory");
    r.value = value;
    r.error = error;
#elif defined(__riscv) && (__riscv_xlen == 64)
    register __usize snum asm("a7") = num;
    register __usize value asm("a0");
    register __usize error asm("a1");
    register __usize r0 asm("a0") = a0;
    register __usize r1 asm("a1") = a1;
    register __usize r2 asm("a2") = a2;
    register __usize r3 asm("a3") = a3;
    register __usize r4 asm("a4") = a4;
    register __usize r5 asm("a5") = a5;
    asm volatile("ecall"
                 : "=r"(value), "=r"(error)
                 : "r"(snum), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5)
                 : "memory");
    r.value = value;
    r.error = error;
#elif defined(__loongarch64)
    register __usize snum asm("a7") = num;
    register __usize value asm("a0");
    register __usize error asm("a1");
    register __usize r0 asm("a0") = a0;
    register __usize r1 asm("a1") = a1;
    register __usize r2 asm("a2") = a2;
    register __usize r3 asm("a3") = a3;
    register __usize r4 asm("a4") = a4;
    register __usize r5 asm("a5") = a5;
    asm volatile("syscall 0"
                 : "=r"(value), "=r"(error)
                 : "r"(snum), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5)
                 : "memory");
    r.value = value;
    r.error = error;
#else
#error "Unsupported architecture!"
#endif
    return r;
}
