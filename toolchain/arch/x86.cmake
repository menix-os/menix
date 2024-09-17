# x86 platform settings

set(CMAKE_C_COMPILER_TARGET x86_64-none-elf)
set(CMAKE_ASM_COMPILER_TARGET x86_64-none-elf)
set(MENIX_BITS 64)
set(MENIX_ARCH_DIR x86)
set(MENIX_ARCH_NAME x86_64)

target_compile_options(common INTERFACE
	-mgeneral-regs-only
	-mno-red-zone
)

target_compile_options(common_kernel INTERFACE
	-mcmodel=kernel
)

# Memory allocation
add_option(page_size NUMBER 0x1000)

# Debug information for page faults
add_option(x86_pf_debug BOOL OFF)

add_option(vm_map_min_addr NUMBER 0x10000)
add_option(vm_map_base NUMBER 0xFFFF900000000000)
add_option(vm_map_foreign_base NUMBER 0xFFFFD00000000000)

add_option(user_stack_size NUMBER 0x200000)
add_option(user_stack_addr NUMBER 0x70000000000)
add_option(user_interp_base NUMBER 0x60000000000)
add_option(kernel_stack_size NUMBER 0x200000)
