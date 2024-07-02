//? Multiboot2 implementation.

#include <menix/boot.h>

#include "multiboot2.h"

static const ATTR(used) ATTR(section(".multiboot")) struct multiboot_header mb_header = {
	.magic = MULTIBOOT2_HEADER_MAGIC,
	.architecture = MULTIBOOT_ARCHITECTURE_I386,
	.header_length = sizeof(struct multiboot_header),
	.checksum = -(uint32_t)(MULTIBOOT2_HEADER_MAGIC + MULTIBOOT_ARCHITECTURE_I386 + sizeof(struct multiboot_header)),
};

void kernel_boot(uint32_t magic, uint32_t addr)
{
	kernel_main();
}
