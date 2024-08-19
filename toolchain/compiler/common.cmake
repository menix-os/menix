# Options that are available in all compilers.

add_compile_options(
	-ffreestanding
	-nostdlib
	-nostartfiles
	-fno-omit-frame-pointer
	-fno-builtin
	-mcmodel=large
)

add_link_options(
	-ffreestanding
	-nostdlib
	-z noexecstack
	-Wl,--build-id=none
)

add_compile_options(-Wall)
