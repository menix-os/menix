# Main kernel sources

add_executable(menix)
target_link_libraries(menix PRIVATE common common_kernel)

target_sources(menix PUBLIC main.c)

# Mark entry point.
target_link_options(menix PUBLIC -e kernel_boot)

add_subdirectory(arch/${MENIX_ARCH})
target_include_directories(menix PRIVATE arch/${MENIX_ARCH}/include)
add_subdirectory(boot)
add_subdirectory(klibc)
add_subdirectory(memory)
add_subdirectory(syscall)
add_subdirectory(system)
add_subdirectory(util)
