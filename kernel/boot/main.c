#include <menix/boot/cmdline.h>
#include <menix/boot/file.h>
#include <menix/boot/init.h>
#include <menix/boot/main.h>
#include <menix/sys/console.h>
#include <stddef.h>

[[__initdata]]
struct boot_file boot_files[32] = {};

[[__initdata]]
size_t boot_files_count = 0;

[[noreturn]]
void kmain() {
    console_write("Hello world!", 12);
    while (1) {}
}
