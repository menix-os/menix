# x86 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(x86
	boot/gdt.c
	boot/tss.c
	boot/interrupts.c
	boot/idt.c
	arch.c
	io/serial.c
	vm.c
	pm.c
	syscall.s
)
