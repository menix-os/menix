# Options required for building with clang.

target_compile_options(common INTERFACE
	-Wno-unused-command-line-argument
	-no-integrated-as
)

target_compile_options(common_kernel INTERFACE
	-flto
)
