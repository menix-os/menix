include(${MENIX_UTIL_PATH})
add_library(menix_arch_x86
	boot/gdt.asm
	boot/idt.asm
	boot/boot.asm
	vga.c
	idt.c
	error.c
	arch.c
)

target_include_directories(menix_arch_x86 PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include/)

# Overwrite linker flags. add_link_options() doesn't respect this.
set(CMAKE_EXE_LINKER_FLAGS "-T ${CMAKE_CURRENT_SOURCE_DIR}/linker.ld" CACHE INTERNAL "")
