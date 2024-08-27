# System call implementations

target_sources(menix PUBLIC
	syscall.c
	syscall_table.c

	do_proc.c
	do_vm.c
)
