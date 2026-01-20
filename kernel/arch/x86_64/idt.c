#include <menix/compiler.h>
#include <stdint.h>

struct [[__packed]] idtr {
    uint16_t limit;
    struct idt* base;
};
