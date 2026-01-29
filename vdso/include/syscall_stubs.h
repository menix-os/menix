#pragma once

#include <menix/errno.h>
#include <stdint.h>

#ifdef __x86_64__
#define MENIX_ASM_REG_NUM "rax"
#define MENIX_ASM_REG_RET "rax"
#define MENIX_ASM_REG_A0  "rdi"
#define MENIX_ASM_REG_A1  "rsi"
#define MENIX_ASM_REG_A2  "rdx"
#define MENIX_ASM_REG_A3  "r9"
#define MENIX_ASM_REG_A4  "r8"
#define MENIX_ASM_REG_A5  "r10"
#define MENIX_ASM_SYSCALL "syscall"
#define MENIX_ASM_CLOBBER "rcx", "r11"
typedef uint64_t menix_arg_t;
#elif defined(__aarch64__)
#define MENIX_ASM_REG_NUM "x8"
#define MENIX_ASM_REG_RET "x0"
#define MENIX_ASM_REG_A0  "x0"
#define MENIX_ASM_REG_A1  "x1"
#define MENIX_ASM_REG_A2  "x2"
#define MENIX_ASM_REG_A3  "x3"
#define MENIX_ASM_REG_A4  "x4"
#define MENIX_ASM_REG_A5  "x5"
#define MENIX_ASM_SYSCALL "svc 0"
#define MENIX_ASM_CLOBBER
typedef uint64_t menix_arg_t;
#elif defined(__riscv) && (__riscv_xlen == 64)
#define MENIX_ASM_REG_NUM "a7"
#define MENIX_ASM_REG_RET "a0"
#define MENIX_ASM_REG_A0  "a0"
#define MENIX_ASM_REG_A1  "a1"
#define MENIX_ASM_REG_A2  "a2"
#define MENIX_ASM_REG_A3  "a3"
#define MENIX_ASM_REG_A4  "a4"
#define MENIX_ASM_REG_A5  "a5"
#define MENIX_ASM_SYSCALL "ecall"
#define MENIX_ASM_CLOBBER
typedef uint64_t menix_arg_t;
#elif defined(__loongarch64)
#define MENIX_ASM_REG_NUM "a7"
#define MENIX_ASM_REG_RET "a0"
#define MENIX_ASM_REG_A0  "a0"
#define MENIX_ASM_REG_A1  "a1"
#define MENIX_ASM_REG_A2  "a2"
#define MENIX_ASM_REG_A3  "a3"
#define MENIX_ASM_REG_A4  "a4"
#define MENIX_ASM_REG_A5  "a5"
#define MENIX_ASM_SYSCALL "syscall 0"
#define MENIX_ASM_CLOBBER
typedef uint64_t menix_arg_t;
#else
#error "Unsupported architecture!"
#endif

static inline menix_errno_t menix_syscall0(menix_arg_t num) {
    register menix_arg_t rnum asm(MENIX_ASM_REG_NUM) = num;
    register menix_arg_t value asm(MENIX_ASM_REG_RET);
    asm volatile(MENIX_ASM_SYSCALL : "=r"(value) : "r"(rnum) : "memory", MENIX_ASM_CLOBBER);

    return value;
}

static inline menix_errno_t menix_syscall1(menix_arg_t num, menix_arg_t a0) {
    register menix_arg_t rnum asm(MENIX_ASM_REG_NUM) = num;
    register menix_arg_t value asm(MENIX_ASM_REG_RET);
    register menix_arg_t r0 asm(MENIX_ASM_REG_A0) = a0;
    asm volatile(MENIX_ASM_SYSCALL : "=r"(value) : "r"(rnum), "r"(r0) : "memory", MENIX_ASM_CLOBBER);

    return value;
}

static inline menix_errno_t menix_syscall2(menix_arg_t num, menix_arg_t a0, menix_arg_t a1) {
    register menix_arg_t rnum asm(MENIX_ASM_REG_NUM) = num;
    register menix_arg_t value asm(MENIX_ASM_REG_RET);
    register menix_arg_t r0 asm(MENIX_ASM_REG_A0) = a0;
    register menix_arg_t r1 asm(MENIX_ASM_REG_A1) = a1;
    asm volatile(MENIX_ASM_SYSCALL : "=r"(value) : "r"(rnum), "r"(r0), "r"(r1) : "memory", MENIX_ASM_CLOBBER);

    return value;
}

static inline menix_errno_t menix_syscall3(menix_arg_t num, menix_arg_t a0, menix_arg_t a1, menix_arg_t a2) {
    register menix_arg_t rnum asm(MENIX_ASM_REG_NUM) = num;
    register menix_arg_t value asm(MENIX_ASM_REG_RET);
    register menix_arg_t r0 asm(MENIX_ASM_REG_A0) = a0;
    register menix_arg_t r1 asm(MENIX_ASM_REG_A1) = a1;
    register menix_arg_t r2 asm(MENIX_ASM_REG_A2) = a2;
    asm volatile(MENIX_ASM_SYSCALL : "=r"(value) : "r"(rnum), "r"(r0), "r"(r1), "r"(r2) : "memory", MENIX_ASM_CLOBBER);

    return value;
}

static inline menix_errno_t menix_syscall4(
    menix_arg_t num,
    menix_arg_t a0,
    menix_arg_t a1,
    menix_arg_t a2,
    menix_arg_t a3
) {
    register menix_arg_t rnum asm(MENIX_ASM_REG_NUM) = num;
    register menix_arg_t value asm(MENIX_ASM_REG_RET);
    register menix_arg_t r0 asm(MENIX_ASM_REG_A0) = a0;
    register menix_arg_t r1 asm(MENIX_ASM_REG_A1) = a1;
    register menix_arg_t r2 asm(MENIX_ASM_REG_A2) = a2;
    register menix_arg_t r3 asm(MENIX_ASM_REG_A3) = a3;
    asm volatile(MENIX_ASM_SYSCALL
                 : "=r"(value)
                 : "r"(rnum), "r"(r0), "r"(r1), "r"(r2), "r"(r3)
                 : "memory", MENIX_ASM_CLOBBER);

    return value;
}

static inline menix_errno_t menix_syscall5(
    menix_arg_t num,
    menix_arg_t a0,
    menix_arg_t a1,
    menix_arg_t a2,
    menix_arg_t a3,
    menix_arg_t a4
) {
    register menix_arg_t rnum asm(MENIX_ASM_REG_NUM) = num;
    register menix_arg_t value asm(MENIX_ASM_REG_RET);
    register menix_arg_t r0 asm(MENIX_ASM_REG_A0) = a0;
    register menix_arg_t r1 asm(MENIX_ASM_REG_A1) = a1;
    register menix_arg_t r2 asm(MENIX_ASM_REG_A2) = a2;
    register menix_arg_t r3 asm(MENIX_ASM_REG_A3) = a3;
    register menix_arg_t r4 asm(MENIX_ASM_REG_A4) = a4;
    asm volatile(MENIX_ASM_SYSCALL
                 : "=r"(value)
                 : "r"(rnum), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4)
                 : "memory", MENIX_ASM_CLOBBER);

    return value;
}

static inline menix_errno_t menix_syscall6(
    menix_arg_t num,
    menix_arg_t a0,
    menix_arg_t a1,
    menix_arg_t a2,
    menix_arg_t a3,
    menix_arg_t a4,
    menix_arg_t a5
) {
    register menix_arg_t rnum asm(MENIX_ASM_REG_NUM) = num;
    register menix_arg_t value asm(MENIX_ASM_REG_RET);
    register menix_arg_t r0 asm(MENIX_ASM_REG_A0) = a0;
    register menix_arg_t r1 asm(MENIX_ASM_REG_A1) = a1;
    register menix_arg_t r2 asm(MENIX_ASM_REG_A2) = a2;
    register menix_arg_t r3 asm(MENIX_ASM_REG_A3) = a3;
    register menix_arg_t r4 asm(MENIX_ASM_REG_A4) = a4;
    register menix_arg_t r5 asm(MENIX_ASM_REG_A5) = a5;
    asm volatile(MENIX_ASM_SYSCALL
                 : "=r"(value)
                 : "r"(rnum), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5)
                 : "memory", MENIX_ASM_CLOBBER);

    return value;
}
