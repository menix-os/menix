//? Multiboot2 implementation.

#include <menix/common.h>

#ifndef CONFIG_arch_x86
#error "Multiboot2 only supports x86!
#endif

#include <menix/arch.h>
#include <menix/boot.h>
#include <menix/log.h>

#include "multiboot2.h"

#define MB2_ENTRY ATTR(used) ATTR(section(".multiboot")) ATTR(aligned(0x10)) static const

MB2_ENTRY struct multiboot_header mb2_header = {
	.magic = MULTIBOOT2_HEADER_MAGIC,
	.architecture = MULTIBOOT_ARCHITECTURE_I386,
	.header_length = sizeof(struct multiboot_header),
	.checksum = -(uint32_t)(MULTIBOOT2_HEADER_MAGIC + MULTIBOOT_ARCHITECTURE_I386 + sizeof(struct multiboot_header)),
};

MB2_ENTRY struct multiboot_tag_efi64 mb2_tag_efi64 = {
	.type = MULTIBOOT_TAG_TYPE_EFI64,
	.size = sizeof(struct multiboot_tag_efi64),
};

MB2_ENTRY struct multiboot_tag mb2_tag_end = {
	.type = MULTIBOOT_HEADER_TAG_END,
	.size = sizeof(struct multiboot_tag),
};

void kernel_boot(uint32_t magic, uint32_t addr)
{
	arch_init();
	boot_log("Booting with MB2 successful!\n");
	boot_log("magic: %u, addr: %u", magic, addr);

	BootInfo info = { 0 };

	kernel_main(&info);
}
