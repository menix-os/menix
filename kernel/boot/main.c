#include <kernel/boot/cmdline.h>
#include <kernel/boot/file.h>
#include <kernel/boot/init.h>
#include <kernel/boot/main.h>
#include <kernel/sys/console.h>
#include <stddef.h>

[[__initdata]]
struct boot_file boot_files[32] = {};
[[__initdata]]
size_t boot_files_count = 0;

[[noreturn]]
void kernel_main() {
    console_write("Hello world!", 12);
    while (1) {}
}
