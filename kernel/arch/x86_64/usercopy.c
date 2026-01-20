#include <menix/arch/usercopy.h>
#include <menix/compiler.h>

extern void arch_usercopy_read_start();
extern void arch_usercopy_read_end();
extern void arch_usercopy_read_fault();

static const struct usercopy_region read_region = {
    .start_ip = arch_usercopy_read_start,
    .end_ip = arch_usercopy_read_end,
    .fault_ip = arch_usercopy_read_fault,
};

[[__naked]]
bool arch_usercopy_read(uint8_t* dst, const __user uint8_t* src, size_t len) {
    asm volatile(
        // Setup regs for `rep movsb`.
        "xchg rcx, rdx\n"
        "mov rax, [rip + %0]\n"
        "mov [rdx], rax\n"

        ".global arch_usercopy_read_start\n"
        "arch_usercopy_read_start:\n"
        "rep movsb\n"

        ".global arch_usercopy_read_end\n"
        "arch_usercopy_read_end:\n"
        "xor rax, rax\n"
        "mov [rdx], rax\n"
        "mov rax, 1\n"
        "ret\n"

        ".global arch_usercopy_read_fault\n"
        "arch_usercopy_read_fault:\n"
        "xor rax, rax\n"
        "mov [rdx], rax\n"
        "ret\n"
        :
        : "i"(&read_region)
        : "memory"
    );
}

extern void arch_usercopy_write_start();
extern void arch_usercopy_write_end();
extern void arch_usercopy_write_fault();

static const struct usercopy_region write_region = {
    .start_ip = arch_usercopy_write_start,
    .end_ip = arch_usercopy_write_end,
    .fault_ip = arch_usercopy_write_fault,
};

[[__naked]]
bool arch_usercopy_write(__user uint8_t* dst, const uint8_t* src, size_t len) {
    asm volatile(
        // Setup regs for `rep movsb`.
        "xchg rcx, rdx\n"
        "mov rax, [rip + %0]\n"
        "mov [rdx], rax\n"

        ".global arch_usercopy_write_start\n"
        "arch_usercopy_write_start:\n"
        "rep movsb\n"

        ".global arch_usercopy_write_end\n"
        "arch_usercopy_write_end:\n"
        "xor rax, rax\n"
        "mov [rdx], rax\n"
        "mov rax, 1\n"
        "ret\n"

        ".global arch_usercopy_write_fault\n"
        "arch_usercopy_write_fault:\n"
        "xor rax, rax\n"
        "mov [rdx], rax\n"
        "ret\n"
        :
        : "i"(&read_region)
        : "memory"
    );
}

extern void arch_usercopy_strlen_start();
extern void arch_usercopy_strlen_end();
extern void arch_usercopy_strlen_fault();

static const struct usercopy_region strlen_region = {
    .start_ip = arch_usercopy_strlen_start,
    .end_ip = arch_usercopy_strlen_end,
    .fault_ip = arch_usercopy_strlen_fault,
};

[[__naked]]
bool arch_usercopy_strlen(const __user uint8_t* str, size_t max, size_t* len) {
    asm volatile(
        "mov rax, [rip + %0]\n"
        "mov [rcx], rax\n"

        ".global arch_usercopy_strlen_start\n"
        "arch_usercopy_strlen_start:\n"
        "xor r8, r8\n"
        ".Lloop:\n"
        "cmp byte ptr [rdi + r8], 0\n"
        "je .Lleave\n"
        "inc r8\n"
        "cmp rsi, r8\n"
        "jne .Lloop\n"
        ".Lleave:\n"
        "mov [rdx], r8\n"

        ".global arch_usercopy_strlen_end\n"
        "arch_usercopy_strlen_end:\n"
        "xor rax, rax\n"
        "mov [rcx], rax\n"
        "mov rax, 1\n"
        "ret\n"

        ".global arch_usercopy_strlen_fault\n"
        "arch_usercopy_strlen_fault:\n"
        "xor rax, rax\n"
        "mov [rcx], rax\n"
        "ret\n"
        :
        : "i"(&read_region)
        : "memory"
    );
}
