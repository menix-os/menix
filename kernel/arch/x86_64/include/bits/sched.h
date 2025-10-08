#pragma once

#include <stdint.h>

struct arch_task_context {
    uint64_t rsp;
    uint8_t* fpu_region;
    uint16_t ds, es, fs, gs;
    uint64_t fs_base, gs_base;
};

struct arch_context {
    uint64_t r15;
    uint64_t r14;
    uint64_t r13;
    uint64_t r12;
    uint64_t r11;
    uint64_t r10;
    uint64_t r9;
    uint64_t r8;
    uint64_t rsi;
    uint64_t rdi;
    uint64_t rbp;
    uint64_t rdx;
    uint64_t rcx;
    uint64_t rbx;
    uint64_t rax;
    uint64_t isr;   // Pushed onto the stack by the interrupt handler stubs.
    uint64_t error; // Pushed onto the stack by the CPU if the interrupt has an error code.
    uint64_t rip;   // The rest is pushed onto the stack by the CPU during an interrupt.
    uint64_t cs;
    uint64_t rflags;
    uint64_t rsp;
    uint64_t ss;
};
