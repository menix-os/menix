#ifndef _MENIX_SYSCALL_STUBS_H
#define _MENIX_SYSCALL_STUBS_H

#include <menix/compiler.h>
#include <menix/status.h>
#include <stdint.h>

__MENIX_CDECL_START

#ifdef __x86_64__
#define ASM_REG_NUM "rax"
#define ASM_REG_RET "rax"
#define ASM_REG_A0  "rdi"
#define ASM_REG_A1  "rsi"
#define ASM_REG_A2  "rdx"
#define ASM_REG_A3  "r9"
#define ASM_REG_A4  "r8"
#define ASM_REG_A5  "r10"
#define ASM_SYSCALL "syscall"
#define ASM_CLOBBER "rcx", "r11"
typedef uint64_t menix_arg_t;
#elif defined(__aarch64__)
#define ASM_REG_NUM "x8"
#define ASM_REG_RET "x0"
#define ASM_REG_A0  "x0"
#define ASM_REG_A1  "x1"
#define ASM_REG_A2  "x2"
#define ASM_REG_A3  "x3"
#define ASM_REG_A4  "x4"
#define ASM_REG_A5  "x5"
#define ASM_SYSCALL "svc 0"
#define ASM_CLOBBER
typedef uint64_t menix_arg_t;
#elif defined(__riscv) && (__riscv_xlen == 64)
#define ASM_REG_NUM "a7"
#define ASM_REG_RET "a0"
#define ASM_REG_A0  "a0"
#define ASM_REG_A1  "a1"
#define ASM_REG_A2  "a2"
#define ASM_REG_A3  "a3"
#define ASM_REG_A4  "a4"
#define ASM_REG_A5  "a5"
#define ASM_SYSCALL "ecall"
#define ASM_CLOBBER
typedef uint64_t menix_arg_t;
#elif defined(__loongarch64)
#define ASM_REG_NUM "a7"
#define ASM_REG_RET "a0"
#define ASM_REG_A0  "a0"
#define ASM_REG_A1  "a1"
#define ASM_REG_A2  "a2"
#define ASM_REG_A3  "a3"
#define ASM_REG_A4  "a4"
#define ASM_REG_A5  "a5"
#define ASM_SYSCALL "syscall 0"
#define ASM_CLOBBER
typedef uint64_t menix_arg_t;
#else
#error "Unsupported architecture!"
#endif

static inline menix_status_t menix_syscall0(menix_arg_t num) {
    register menix_arg_t rnum asm(ASM_REG_NUM) = num;
    register menix_arg_t value asm(ASM_REG_RET);
    asm volatile(ASM_SYSCALL : "=r"(value) : "r"(rnum) : "memory", ASM_CLOBBER);

    return value;
}

static inline menix_status_t menix_syscall1(menix_arg_t num, menix_arg_t a0) {
    register menix_arg_t rnum asm(ASM_REG_NUM) = num;
    register menix_arg_t value asm(ASM_REG_RET);
    register menix_arg_t r0 asm(ASM_REG_A0) = a0;
    asm volatile(ASM_SYSCALL : "=r"(value) : "r"(rnum), "r"(r0) : "memory", ASM_CLOBBER);

    return value;
}

static inline menix_status_t menix_syscall2(menix_arg_t num, menix_arg_t a0, menix_arg_t a1) {
    register menix_arg_t rnum asm(ASM_REG_NUM) = num;
    register menix_arg_t value asm(ASM_REG_RET);
    register menix_arg_t r0 asm(ASM_REG_A0) = a0;
    register menix_arg_t r1 asm(ASM_REG_A1) = a1;
    asm volatile(ASM_SYSCALL : "=r"(value) : "r"(rnum), "r"(r0), "r"(r1) : "memory", ASM_CLOBBER);

    return value;
}

static inline menix_status_t menix_syscall3(menix_arg_t num, menix_arg_t a0, menix_arg_t a1, menix_arg_t a2) {
    register menix_arg_t rnum asm(ASM_REG_NUM) = num;
    register menix_arg_t value asm(ASM_REG_RET);
    register menix_arg_t r0 asm(ASM_REG_A0) = a0;
    register menix_arg_t r1 asm(ASM_REG_A1) = a1;
    register menix_arg_t r2 asm(ASM_REG_A2) = a2;
    asm volatile(ASM_SYSCALL : "=r"(value) : "r"(rnum), "r"(r0), "r"(r1), "r"(r2) : "memory", ASM_CLOBBER);

    return value;
}

static inline menix_status_t menix_syscall4(
    menix_arg_t num,
    menix_arg_t a0,
    menix_arg_t a1,
    menix_arg_t a2,
    menix_arg_t a3
) {
    register menix_arg_t rnum asm(ASM_REG_NUM) = num;
    register menix_arg_t value asm(ASM_REG_RET);
    register menix_arg_t r0 asm(ASM_REG_A0) = a0;
    register menix_arg_t r1 asm(ASM_REG_A1) = a1;
    register menix_arg_t r2 asm(ASM_REG_A2) = a2;
    register menix_arg_t r3 asm(ASM_REG_A3) = a3;
    asm volatile(ASM_SYSCALL : "=r"(value) : "r"(rnum), "r"(r0), "r"(r1), "r"(r2), "r"(r3) : "memory", ASM_CLOBBER);

    return value;
}

static inline menix_status_t menix_syscall5(
    menix_arg_t num,
    menix_arg_t a0,
    menix_arg_t a1,
    menix_arg_t a2,
    menix_arg_t a3,
    menix_arg_t a4
) {
    register menix_arg_t rnum asm(ASM_REG_NUM) = num;
    register menix_arg_t value asm(ASM_REG_RET);
    register menix_arg_t r0 asm(ASM_REG_A0) = a0;
    register menix_arg_t r1 asm(ASM_REG_A1) = a1;
    register menix_arg_t r2 asm(ASM_REG_A2) = a2;
    register menix_arg_t r3 asm(ASM_REG_A3) = a3;
    register menix_arg_t r4 asm(ASM_REG_A4) = a4;
    asm volatile(ASM_SYSCALL
                 : "=r"(value)
                 : "r"(rnum), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4)
                 : "memory", ASM_CLOBBER);

    return value;
}

static inline menix_status_t menix_syscall6(
    menix_arg_t num,
    menix_arg_t a0,
    menix_arg_t a1,
    menix_arg_t a2,
    menix_arg_t a3,
    menix_arg_t a4,
    menix_arg_t a5
) {
    register menix_arg_t rnum asm(ASM_REG_NUM) = num;
    register menix_arg_t value asm(ASM_REG_RET);
    register menix_arg_t r0 asm(ASM_REG_A0) = a0;
    register menix_arg_t r1 asm(ASM_REG_A1) = a1;
    register menix_arg_t r2 asm(ASM_REG_A2) = a2;
    register menix_arg_t r3 asm(ASM_REG_A3) = a3;
    register menix_arg_t r4 asm(ASM_REG_A4) = a4;
    register menix_arg_t r5 asm(ASM_REG_A5) = a5;
    asm volatile(ASM_SYSCALL
                 : "=r"(value)
                 : "r"(rnum), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5)
                 : "memory", ASM_CLOBBER);

    return value;
}

__MENIX_CDECL_END
#endif
