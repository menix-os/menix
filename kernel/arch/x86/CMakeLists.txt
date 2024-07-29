# x86 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(x86
	arch.c
	boot/gdt.c
	boot/tss.c
	boot/syscall_handler.s
	boot/interrupts.c
	boot/idt.c
	io/serial.c
	io/pci.c
	io/io.c
)
add_option(page_size 0x1000)
add_option(bits ${MENIX_BITS})
require_option(efi)
