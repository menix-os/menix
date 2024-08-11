# System call implementations

include(${MENIX_UTIL_PATH})
add_library(menix_sys
	syscall_table.c
	do_null.c
	do_mmap.c
)