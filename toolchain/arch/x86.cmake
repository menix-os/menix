set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_C_COMPILER_TARGET x86_64-none-elf)
set(CMAKE_ASM_COMPILER_TARGET x86_64-none-elf)
set(MENIX_BITS 64)
set(MENIX_ARCH_DIR x86)

add_compile_options(-ffreestanding -nostdlib -nostdinc -fPIC -nostartfiles -mgeneral-regs-only)
add_link_options(-ffreestanding -nostdlib -z noexecstack)

include_directories(/usr/include/efi/)
