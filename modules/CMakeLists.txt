# Modules

set(MENIX_PARENT_CAT menix_builtin)
add_library(${MENIX_PARENT_CAT} INTERFACE)
include(${MENIX_UTIL_PATH})

# Prepare module build folder so we can create an archive, even if there's no modules to be packed.
file(MAKE_DIRECTORY "${CMAKE_BINARY_DIR}/bin/mod")

# Architecture dependent drivers
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/arch/${MENIX_ARCH})
	add_subdirectory(arch/${MENIX_ARCH})
endif()

# Shared headers
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

# Generic drivers
add_subdirectory(drv)

# File systems
add_subdirectory(fs)

# Example modules, disabled by default
add_subdirectory(examples/cpp_example)
add_subdirectory(examples/rust_example)
