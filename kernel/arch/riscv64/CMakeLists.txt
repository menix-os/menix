# riscv64 architecture sources

target_sources(menix PUBLIC
	system/arch.c
	system/arch.s
	sch/core.c
	sch/thread.c
	memory/vm.c
)
