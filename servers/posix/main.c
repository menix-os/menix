#include <menix/status.h>
#include <menix/system.h>

int main() {
    menix_log("Hello, server world!\n", 21);
    menix_panic(MENIX_ERR_INTERNAL);
}
