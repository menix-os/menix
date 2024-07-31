# Options required for building with clang.

set(CMAKE_SYSTEM_NAME Generic)

add_compile_options(
	-ffreestanding
	-nostdlib
	-fPIC
	-nostartfiles
	-mgeneral-regs-only
	-fno-omit-frame-pointer
	-fno-builtin)

add_link_options(
	-ffreestanding
	-nostdlib
	-z noexecstack
	-Wl,--build-id=none
)

add_compile_options(-Wall -Wno-unused-command-line-argument)
