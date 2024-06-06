# Adding a new CPU architecture target

## Toolchain file
Create a new file in `/kernel/toolchain/arch/<cpu>.cmake`.
Use this template and fill in the `<placeholders>`:

```cmake
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_C_COMPILER clang)				# Path to the clang binary
set(CMAKE_C_COMPILER_TARGET <triple>)	# C Target triple, e.g. "i386-none-elf"
set(CMAKE_ASM_COMPILER_TARGET <triple>)	# ASM Target triple, should be the same as C
set(MENIX_BITS <bits>)					# Word size in bits. 32, 64 or 128
set(MENIX_ARCH_DIR <dir>) 				# CPU specific code directory, relative to /kernel/arch/

# Architecture specific compiler/linker flags
```

The following flags are quite common. You can copy these as well, if they're applicable:

```cmake
add_compile_options(-Wno-unused-command-line-argument)		# Ignore assembler warnings for CMake args
add_compile_options(-ffreestanding -nostdlib)				# Freestanding binary (compiler)
add_link_options(-ffreestanding -nostdlib -z noexecstack)	# Freestanding binary (linker)
```

## Arch file
Create a directory in `/kernel/arch/<dir>`. The name should match what you set `MENIX_ARCH_DIR` to.
Create a `CMakeLists.txt` and fill it with this template:

```cmake
include(${MENIX_UTIL_PATH})
add_library(menix_arch_<arch>
	<src>
)

target_include_directories(menix_arch_<arch> PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include/)
set(CMAKE_EXE_LINKER_FLAGS "-T ${CMAKE_CURRENT_SOURCE_DIR}/linker.ld" CACHE INTERNAL "")
```

## Boot setup
Almost every platform requires some form of bootstrapping. `menix` achieves this with a boot assembly stub that
bootstraps the stack and calls the C entry point.
This stub should be located in `/kernel/arch/<arch>/boot/entry.asm`.

## Linker script
Create a file called `linker.ld`. Here, the platform specific kernel layout should be determined.
You can use the x86 linker script as a reference.

## Device trees
On some platforms, you need to use a device tree. In `menix`, these get passed by the boot loader, like
U-Boot or GRUB. In this case, add `require_option(device_tree)` to `/kernel/arch/<arch>/CMakeLists.txt`.

## Example structure
The final structure should look something like this:
```
/kernel/arch/<arch>/boot/entry.asm
				   /CMakeLists.txt
				   /linker.ld
	   /toolchain/arch/<arch>.cmake
```
