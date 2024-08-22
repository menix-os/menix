# Options that are available in all compilers.

add_compile_options(
	-ffreestanding
	-nostdlib
	-fno-omit-frame-pointer
	-fno-builtin
)

add_link_options(
	-ffreestanding
	-nostdlib
	-nostartfiles
	-z noexecstack
	-Wl,--build-id=none
)

add_compile_options(-Wall)
