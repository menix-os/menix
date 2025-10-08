#pragma once

#include <stdint.h>

static inline uint64_t asm_rdmsr(uint32_t msr) {
    uint32_t eax;
    uint32_t edx;
    asm volatile("rdmsr" : "=a"(eax), "=d"(edx) : "c"(msr) : "memory");
    return ((uint64_t)edx << 32) | eax;
}

// Writes a 64-bit value to a given MSR.
static inline void asm_wrmsr(uint32_t msr, uint64_t val) {
    uint32_t eax = (uint32_t)val;
    uint32_t edx = val >> 32;
    asm volatile("wrmsr" : : "a"(eax), "d"(edx), "c"(msr) : "memory");
}

// Writes a 64-bit value to a control register using XSETBV.
static inline void asm_wrxcr(uint32_t reg, uint64_t val) {
    uint32_t eax = val;
    uint32_t edx = val >> 32;
    asm volatile("xsetbv" ::"a"(eax), "d"(edx), "c"(reg) : "memory");
}

// Saves the FPU state to a 512-byte region of memory using FXSAVE.
// Pointer must be 16-byte aligned.
static inline void asm_fxsave(void* mem) {
    asm volatile("fxsave %0" : "+m"(*(uint8_t*)mem)::"memory");
}

// Restores the FPU state from a 512-byte region of memory using FXRSTOR.
// Pointer must be 16-byte aligned.
static inline void asm_fxrstor(void* mem) {
    asm volatile("fxrstor %0" ::"m"(*(uint8_t*)mem) : "memory");
}

// Saves the FPU state to a region of memory using XSAVE.
// Pointer must be 16-byte aligned.
static inline void asm_xsave(void* mem) {
    asm volatile("xsave %0" : "+m"(*(uint8_t*)mem)::"memory");
}

// Restores the FPU state from a region of memory using XRSTOR.
// Pointer must be 16-byte aligned.
static inline void asm_xrstor(void* mem) {
    asm volatile("xrstor %0" ::"m"(*(uint8_t*)mem) : "memory");
}

static inline uint8_t asm_inb(uint16_t port) {
    uint8_t result;
    asm volatile("in %0, %1" : "=a"(result) : "Nd"(port));
    return result;
}

static inline uint16_t asm_inw(uint16_t port) {
    uint16_t result;
    asm volatile("in %0, %1" : "=a"(result) : "Nd"(port));
    return result;
}

static inline uint32_t asm_inl(uint16_t port) {
    uint32_t result;
    asm volatile("in %0, %1" : "=a"(result) : "Nd"(port));
    return result;
}

static inline void asm_outb(uint16_t port, uint8_t value) {
    asm volatile("out %0, %1" : : "Nd"(port), "a"(value));
}

static inline void asm_outw(uint16_t port, uint16_t value) {
    asm volatile("out %0, %1" : : "Nd"(port), "a"(value));
}

static inline void asm_outl(uint16_t port, uint32_t value) {
    asm volatile("out %0, %1" : : "Nd"(port), "a"(value));
}
