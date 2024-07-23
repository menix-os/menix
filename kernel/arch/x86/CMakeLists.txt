# x86 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(x86
	boot/gdt.c
	boot/syscall_handler.s
	boot/interrupts.c
	boot/idt.c
	boot/init.c
	io/serial.c
	io/pci.c
	io/io.c
)
require_option(efi)
