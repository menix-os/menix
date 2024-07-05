# ? x86 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(x86
	boot/gdt.c
	boot/interrupts.s
	boot/idt.c
	arch.c
	vga.c
	pci.c
)
require_option(efi)
