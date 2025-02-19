# Main kernel sources

target_sources(menix PUBLIC main.c)

set_target_properties(menix PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")

# Mark entry point.
target_link_options(menix PUBLIC -e kernel_boot)

add_subdirectory(arch/${MENIX_ARCH})
add_subdirectory(boot)
add_subdirectory(fs)
add_subdirectory(klibc)
add_subdirectory(memory)
add_subdirectory(syscall)
add_subdirectory(system)
add_subdirectory(util)
