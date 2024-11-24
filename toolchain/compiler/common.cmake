# Options that are available in all compilers.

# Common options
target_compile_options(common INTERFACE
	-ffreestanding
	-fno-omit-frame-pointer
	-Wall
	-Werror
)
target_link_options(common INTERFACE
	-nostdlib
	"SHELL:-z noexecstack"
	-Wl,--build-id=none
)

# Modular kernel modules
target_compile_options(common_ko INTERFACE
	-fPIC
)
target_link_options(common_ko INTERFACE
	-shared
)

# Kernel options
target_compile_options(common_kernel INTERFACE
	-fno-lto
	-fno-PIC
	-fno-PIE
)
target_link_options(common_kernel INTERFACE
	-static
)
