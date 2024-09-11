# Options that are available in all compilers.

# Common options
target_compile_options(common INTERFACE
	-ffreestanding
	-nostdlib
	-fno-omit-frame-pointer
	-fno-builtin
	-fno-stack-protector
	-fno-stack-check
	-Wall
)
target_link_options(common INTERFACE
	-nostdlib
	-nostartfiles
	"SHELL:-z max-page-size=${page_size}"
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
