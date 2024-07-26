// self.h Implementation

#include <menix/util/self.h>

static Elf_Hdr* self_kernel_addr = NULL;

void self_set_kernel(Elf_Hdr* addr)
{
	self_kernel_addr = addr;
}

Elf_Hdr* self_get_kernel()
{
	return self_kernel_addr;
}
