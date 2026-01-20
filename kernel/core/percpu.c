#include <menix/compiler.h>
#include <menix/percpu.h>

[[__used, __section(".percpu")]]
struct percpu percpu_bsp = {
    .self = &percpu_bsp,
    .id = 0,
    .online = false,
};

struct percpu* percpu_new() {
    // TODO
    return nullptr;
}
