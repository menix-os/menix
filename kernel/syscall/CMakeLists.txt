# System call implementations

target_sources(menix PUBLIC
	syscall.c

	memory.c
	process.c
	socket.c
	system.c
	vfs.c
)
