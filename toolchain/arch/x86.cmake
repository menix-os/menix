set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_C_COMPILER clang)
set(CMAKE_C_COMPILER_TARGET i386-none-elf)
set(CMAKE_ASM_COMPILER_TARGET i386-none-elf)
set(MENIX_BITS 32)
set(MENIX_ARCH_DIR x86)

add_compile_options(-Wno-unused-command-line-argument)

# TODO: Port stdarg.h, then add -nostdinc here and move headers one dir up.
add_compile_options(-ffreestanding -nostdlib)

# TODO: Workaround because clang's built-in as has a few bugs in x86 Intel syntax.
add_compile_options(-fno-integrated-as)

add_link_options(-ffreestanding -nostdlib -z noexecstack)
