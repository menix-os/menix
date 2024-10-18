# System call implementations

target_sources(menix PUBLIC
	syscall.c
	syscall_table.c

	write.c
	read.c
	open.c
	close.c
	ioctl.c

	exit.c
	fork.c
	execve.c
	kill.c
	pid.c

	mmap.c
	munmap.c
	mprotect.c

	uname.c
)
