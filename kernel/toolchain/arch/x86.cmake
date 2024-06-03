set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_C_COMPILER clang)
set(CMAKE_C_COMPILER_TARGET i386-none-elf)
set(CMAKE_ASM_COMPILER_TARGET i386-none-elf)
set(MENIX_BITS 32)
set(MENIX_ARCH_DIR x86)

add_compile_options(-ffreestanding -nostdlib)
add_compile_options(-Wno-unused-command-line-argument)
add_link_options(-ffreestanding -nostdlib -z noexecstack)
# Pass --build-id=none to the linker to stop it from creating a ".note.GNU-stack" section.
add_link_options(-Wl,--build-id=none)
