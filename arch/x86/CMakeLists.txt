# ? x86 architecture sources

include(${MENIX_UTIL_PATH})
add_library(menix_arch_x86
	boot/entry.s
	boot/gdt.s
	boot/idt.s
	boot/idt.c
	boot/arch.c
	vga.c
)

set(CMAKE_EXE_LINKER_FLAGS "-T ${CMAKE_CURRENT_SOURCE_DIR}/linker.ld" CACHE INTERNAL "")
