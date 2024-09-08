# System call implementations

target_sources(menix PUBLIC
	syscall.c
	syscall_table.c

	do_fs.c
	do_mod.c
	do_proc.c
	do_uname.c
	do_vm.c
)
