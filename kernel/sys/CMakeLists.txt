# System call implementations

target_sources(menix PUBLIC
	syscall_table.c
	syscall_handler.c
	do_null.c
	do_mmap.c
	do_exec.c
)