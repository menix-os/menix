# Kernel C library
# ! Note that while this library exposes many standard C functions, their
# ! implementation is not guaranteed to be the same as the user-space libc.

add_library(menix_klibc STATIC
	string.c
	stdlib.c
	stdio.c
	log.c
)
