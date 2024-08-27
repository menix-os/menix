# x86 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(x86
	boot/gdt.c
	boot/tss.c
	boot/interrupts.c
	interrupts.s
	boot/idt.c
	arch.c
	io/serial.c
	memory/vm.c
	memory/pm.c
	syscall.s
	memory/slab.c
)
