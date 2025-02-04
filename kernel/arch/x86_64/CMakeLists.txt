# x86 architecture sources

target_sources(menix PRIVATE
	system/gdt.c
	system/idt.c
	system/tss.c
	system/serial.c
	system/arch.s
	system/archctl.c
	system/arch.c
	system/apic.c
	system/interrupts.c
	memory/vm.c
	sch/core.c
	sch/thread.c
)
