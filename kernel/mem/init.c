#include "menix/init.h"
#include <menix/mem.h>

struct phys_mem mem_map[128];
usize mem_map_size = 0;
phys_t mem_kernel_phys_base = 0;
virt_t mem_kernel_virt_base = 0;
virt_t mem_hhdm_base = 0;
char cmdline_buffer[256];
