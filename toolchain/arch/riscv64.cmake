# riscv64 platform settings

set(CMAKE_C_COMPILER_TARGET riscv64-none-elf)
set(CMAKE_ASM_COMPILER_TARGET riscv64-none-elf)

target_compile_options(common INTERFACE
	-march=rv64gc_zicsr_zifencei_zihintpause
)

target_compile_options(common_kernel INTERFACE
	-mcmodel=medany
)
