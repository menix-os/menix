# System call implementations

include(${MENIX_UTIL_PATH})
add_library(menix_sys
	syscall_table.c
	syscall_handler.c
	do_null.c
	do_mmap.c
	do_exec.c
)