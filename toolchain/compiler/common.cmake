# Options that are available in all compilers.

# Common options
target_compile_options(common INTERFACE
	-fno-stack-protector
	-fno-omit-frame-pointer
	-Wall

	# TODO: kernel/system/interrupts.c:52:21: warning: array subscript 1024 is above array bounds of ‘CpuInfo[1024]’ [-Warray-bounds=]
	# -Werror
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
	-fno-PIC
	-fno-PIE
)
target_link_options(common_kernel INTERFACE
	-nostdlib
	-static
	-T ${MENIX_SRC}/toolchain/linker/kernel.ld
)

# Server options
target_compile_options(common_server INTERFACE
	-static
)
target_link_options(common_server INTERFACE
	-static
)
