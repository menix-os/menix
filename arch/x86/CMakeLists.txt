include(${MENIX_UTIL_PATH})
add_library(menix_arch_x86
	boot/entry.s
	boot/gdt.s
	boot/idt.s
	boot/idt.c
	boot/arch.c
	error.c
	vga.c
)

target_include_directories(menix_arch_x86 PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include/)
set(CMAKE_EXE_LINKER_FLAGS "-T ${CMAKE_CURRENT_SOURCE_DIR}/linker.ld" CACHE INTERNAL "")
