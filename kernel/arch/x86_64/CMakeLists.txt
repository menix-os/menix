# x86 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(x86_64
	system/gdt.c
	system/idt.c
	system/tss.c
	system/serial.c
	system/arch.s
	system/arch.c
	system/apic.c
	system/interrupts.c
	system/acpi/hpet.c
	memory/vm.c
	thread/scheduler.c
	thread/thread.c
)
