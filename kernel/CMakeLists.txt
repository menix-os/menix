# Main kernel sources

add_executable(menix
	main.c
)

set_target_properties(menix PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/")

# Mark entry point.
target_link_options(menix PUBLIC -e kernel_boot)

set(MENIX_CURRENT_MOD menix CACHE INTERNAL "")
require_option(license_LGPL)

add_subdirectory(arch/${MENIX_ARCH_DIR})
add_subdirectory(boot)
add_subdirectory(drv)
add_subdirectory(io)
add_subdirectory(fs)
add_subdirectory(klibc)
add_subdirectory(memory)
add_subdirectory(sys)
add_subdirectory(thread)
add_subdirectory(util)
add_subdirectory(video)

# Boot protocols are defined as a module.
target_link_libraries(menix PUBLIC menix_boot)
