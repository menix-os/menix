#include <menix/menix.h>

static menix_handle_t handle_buf[16];
static uint8_t data_buf[4096];

void ipc_test() {
    menix_port_connect(0, handle_buf, 16, data_buf, sizeof(data_buf));
}

int main(int argc, const char** argv) {
    menix_log("Hello, server world!\n", 21);

    return 0;
}
