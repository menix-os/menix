# aarch64 platform settings

set(CMAKE_C_COMPILER_TARGET aarch64-none-elf)
set(CMAKE_ASM_COMPILER_TARGET aarch64-none-elf)

target_compile_options(common INTERFACE
)

target_compile_options(common_kernel INTERFACE
	-mcmodel=medany
)
