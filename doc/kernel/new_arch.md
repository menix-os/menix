# Adding a new CPU architecture target

## Toolchain file
Create a new file in `/toolchain/arch/<arch>.cmake`.
Use this template and fill in the `<placeholders>`:

```cmake
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_C_COMPILER_TARGET <triple>)	# C Target triple, e.g. "i386-none-elf"
set(CMAKE_ASM_COMPILER_TARGET <triple>)	# ASM Target triple, should be the same as C
set(MENIX_BITS <bits>)					# Word size in bits. 32 or 64
set(MENIX_ARCH_DIR <dir>) 				# CPU specific code directory, relative to /kernel/arch/

# Architecture specific compiler/linker flags
```

The following flags are quite common. You can copy these as well, if they're applicable:

```cmake
add_compile_options(-Wno-unused-command-line-argument)		# Ignore assembler warnings for CMake args
add_compile_options(-ffreestanding -nostdlib)				# Freestanding binary (compiler)
add_link_options(-ffreestanding -nostdlib -z noexecstack)	# Freestanding binary (linker)
```

## Architecture file
Create a directory in `/kernel/arch/<arch>`. The name should match what you have set `MENIX_ARCH_DIR` to.

Create a `CMakeLists.txt` and fill it with this template:
```cmake
include(${MENIX_UTIL_PATH})
add_architecture(
	<arch> # Name of the architecture
	<src>  # Source files
)
```

## Boot setup
Almost every platform requires some form of bootstrapping. `menix` makes the bootloader call three functions inside
its entry point: `arch_early_init`, `arch_init` and `arch_stop`.

`arch_early_init`: Called first, to make sure everything is setup correctly.

`arch_init`: Called once the `BootInfo` structure has been filled. This is used for e.g. MMU initialization, subsystem
initialization or to start up other CPU cores.

`arch_stop`: Called after `kernel_main` returns. Here, things like additional cores and running tasks should be stopped.
This function may also be called if a shutdown has been requested.

## Linker script
Create a file called `linker.ld`. Here, the platform specific kernel layout should be determined.
You can use the x86 linker script as a reference.

## Example structure
The final structure should look something like this:
```
kernel
|-	arch
	|-	<arch>
		|-	CMakeLists.txt
		|-	linker.ld
toolchain
|-	arch
	|-	<arch>.cmake
```
