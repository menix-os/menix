# ? x86 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(x86
	boot/gdt.c
	boot/interrupts.s
	boot/idt.c
	boot/arch.c
	io/serial.c
	io/pci.c
	io/io.c
)
require_option(efi)
