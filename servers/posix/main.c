#include <menix/handle.h>
#include <menix/ipc.h>
#include <menix/menix.h>

static menix_handle_t handle_buf[16];
static uint8_t data_buf[4096];

int main(int argc, const char** argv) {
    menix_log("Hello, server world!\n", 21);

    return 0;
}
