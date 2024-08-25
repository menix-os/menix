# Options that are available in all compilers.

add_compile_options(
	-ffreestanding
	-nostdlib
	-fno-omit-frame-pointer
	-fno-builtin
	-fno-stack-protector
	-fno-stack-check
	-fno-lto
	-fno-PIC
)

add_link_options(
	-static
	-nostdlib
	-nostartfiles
	"SHELL:-z max-page-size=${page_size}"
	"SHELL:-z noexecstack"
	-Wl,--build-id=none
)

add_compile_options(-Wall)
