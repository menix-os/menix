# x86 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(x86
	boot/gdt.c
	boot/tss.c
	boot/syscall_handler.s
	boot/interrupts.c
	boot/idt.c
	arch.c
	serial.c
	vm.c
	pm.c
)

add_option(page_size NUMBER 0x1000)

# add_option(smp BOOL ON)
add_option(max_cpus NUMBER 128)

require_option(efi)
require_option(acpi)
