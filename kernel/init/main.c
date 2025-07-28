#include <menix/init.h>
#include <menix/init/file.h>
#include <menix/types.h>

__initdata struct boot_file boot_files[32] = {};
__initdata usize boot_files_count = 0;

__noreturn void kmain() {
	while (1) {}
}
