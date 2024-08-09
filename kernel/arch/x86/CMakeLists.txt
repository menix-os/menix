# x86 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(x86
	boot/gdt.c
	boot/tss.c
	boot/syscall_handler.s
	boot/interrupts.c
	boot/idt.c
	arch.c
	io/serial.c
	vm.c
	pm.c
)
