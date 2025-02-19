# Adding a new CPU architecture target

## Toolchain file
Create a new file in `/toolchain/arch/<arch>.cmake`.
Use this template and fill in the `<placeholders>`:

```cmake
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_C_COMPILER_TARGET <triple>)	# C Target triple, e.g. "i386-none-elf"
set(CMAKE_ASM_COMPILER_TARGET <triple>)	# ASM Target triple, should be the same as C
```

# Architecture specific compiler/linker flags

```cmake
target_compile_options(common INTERFACE
	# Additional flags used by all code
)
target_compile_options(common_kernel INTERFACE
	# Additional flags used by the kernel/built-in modules
)
target_compile_options(common_ko INTERFACE
	# Additional flags used by dynamic kernel modules
)
```

## Architecture file
Create a directory in `/kernel/arch/<arch>`. The name should match what you have set `MENIX_ARCH` to.

Create a `CMakeLists.txt` and fill it with this:
```cmake
target_sources(menix PUBLIC
	<src>  # Source files
)
```

## Example structure
The final structure should look something like this:
```
kernel
|-	arch
	|-	<arch>
		|-	CMakeLists.txt
toolchain
|-	arch
	|-	<arch>.cmake
```
