# x86 platform settings

set(CMAKE_C_COMPILER_TARGET x86_64-none-elf)
set(CMAKE_ASM_COMPILER_TARGET x86_64-none-elf)
set(MENIX_BITS 64)
set(MENIX_ARCH_DIR x86_64)
set(MENIX_ARCH_NAME x86_64)

target_compile_options(common INTERFACE
	-mgeneral-regs-only
	-mno-red-zone
)

target_compile_options(common_kernel INTERFACE
	-mcmodel=kernel
)

set(max_page_size 0x1000)

add_option(x86_pf_debug BOOL OFF)
add_option(vm_map_min_addr NUMBER 0x10000)
add_option(vm_map_base NUMBER 0xFFFF900000000000)
add_option(vm_map_foreign_base NUMBER 0xFFFFD00000000000)
