#include <menix/init/init.h>
#include <menix/init/cmdline.h>
#include <menix/init/file.h>

#include <stddef.h>

__initdata struct boot_file boot_files[32] = {};
__initdata size_t boot_files_count = 0;

void __noreturn kmain() {
    while (1) {}
}
