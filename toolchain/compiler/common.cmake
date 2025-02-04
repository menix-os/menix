# Options that are available in all compilers.

# Common options
target_compile_options(common INTERFACE
	-fno-stack-protector
	-fno-omit-frame-pointer
	-Wall
	-Werror
)
target_link_options(common INTERFACE
	"SHELL:-z noexecstack"
	-Wl,--build-id=none
)

# Kernel options
target_compile_options(common_kernel INTERFACE
	-static
	-nostdlib
	-ffreestanding
	-fno-lto
	-fno-PIC
	-fno-PIE
)

target_link_options(common_kernel INTERFACE
	-nostdlib
	-T ${MENIX_SRC}/toolchain/linker/kernel.ld
)
