# System call implementations

target_sources(menix PUBLIC
	syscall.c

	archctl.c
	chdir.c
	close.c
	execve.c
	exit.c
	faccessat.c
	fchdir.c
	fork.c
	fstat.c
	futex_wait.c
	futex_wake.c
	getcwd.c
	getpid.c
	gettid.c
	ioctl.c
	kill.c
	mkdirat.c
	mmap.c
	mprotect.c
	munmap.c
	openat.c
	read.c
	readtimer.c
	savetls.c
	seek.c
	sigsuspend.c
	uname.c
	waitpid.c
	write.c
)
