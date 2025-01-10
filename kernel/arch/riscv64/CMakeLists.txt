# riscv64 architecture sources

include(${MENIX_UTIL_PATH})
add_architecture(riscv64
	system/arch.c
	system/arch.s
	sch/core.c
	sch/thread.c
	memory/vm.c
)
