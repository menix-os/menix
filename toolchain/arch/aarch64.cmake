# aarch64 platform settings

set(CMAKE_C_COMPILER_TARGET aarch64-none-elf)
set(CMAKE_ASM_COMPILER_TARGET aarch64-none-elf)
set(MENIX_BITS 64)
set(MENIX_ARCH_DIR aarch64)
set(MENIX_ARCH_NAME aarch64)

target_compile_options(common INTERFACE
)

target_compile_options(common_kernel INTERFACE
	-mcmodel=medany
)

set(max_page_size 0x10000)
