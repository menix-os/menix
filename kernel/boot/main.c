#include <menix/boot/cmdline.h>
#include <menix/boot/file.h>
#include <menix/boot/main.h>

#include <stddef.h>

[[__initdata]]
struct boot_file boot_files[32] = {};

[[__initdata]]
size_t boot_files_count = 0;

[[noreturn]]
void kmain() {
    while (1) {}
}
