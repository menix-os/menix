#include <menix/cmdline.h>
#include <menix/init.h>
#include <menix/mem/init.h>

__initdata struct phys_mem mem_map[128];
__initdata usize mem_map_size = 0;
__initdata phys_t mem_kernel_phys_base = 0;
__initdata virt_t mem_kernel_virt_base = 0;
__initdata virt_t mem_hhdm_base = 0;
__initdata char cmdline_buffer[256];
