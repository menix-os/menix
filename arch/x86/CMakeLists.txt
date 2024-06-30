# ? x86 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(x86
	boot/gdt.s
	boot/idt.s
	boot/idt.c
	boot/arch.c
	vga.c
)
