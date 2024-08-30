# x86 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(x86
	boot/gdt.c
	boot/idt.c
	boot/tss.c
	arch.s
	arch.c
	interrupts.c
	io/serial.c
	memory/vm.c
	memory/pm.c
	thread/elf.c
)
