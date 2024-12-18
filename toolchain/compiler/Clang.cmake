# Options required for building with clang.

target_compile_options(common INTERFACE
	-Wno-unused-command-line-argument
	-no-integrated-as
)