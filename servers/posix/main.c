#include <menix/syscalls.h>

void _start() {
    menix_action_await();
    menix_panic();
}
