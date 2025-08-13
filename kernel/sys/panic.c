#include <kernel/sys/panic.h>

void panic(const char* msg, ...) {
    while (1)
        ;
}
