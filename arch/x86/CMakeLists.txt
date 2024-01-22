cmake_minimum_required(VERSION 3.25)
add_library(menix_arch_x86 STATIC
        boot/boot.s
        tty.c
)
target_include_directories(menix_arch_x86 PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include/)
set(CMAKE_EXE_LINKER_FLAGS "-T ${CMAKE_CURRENT_SOURCE_DIR}/linker.ld" CACHE INTERNAL "")
add_compile_definitions(_MENIX_ARCH "${MENIX_ARCH}")
add_compile_definitions(_MENIX_32_BITS)