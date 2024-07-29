# Main configuration file

cmake_minimum_required(VERSION 3.25)

# Project root path.
set(MENIX_SRC ${CMAKE_CURRENT_SOURCE_DIR})

# Project license.
set(MENIX_LICENSE "LGPL")

# Target architecture.
if(NOT DEFINED MENIX_ARCH)
	# Default to x86 if none given.
	set(MENIX_ARCH "x86")
	message(STATUS "Using \"${MENIX_ARCH}\" as default architecture")
endif()

set(CMAKE_C_STANDARD 23)

# Set toolchain, only using clang for now.
set(CMAKE_C_COMPILER clang)
include(${MENIX_SRC}/toolchain/kernel.cmake)
include(${MENIX_SRC}/toolchain/arch/${MENIX_ARCH}.cmake)
include(${MENIX_SRC}/toolchain/arch/defaults.cmake)

project(menix-kernel
	VERSION 0.1.3
	LANGUAGES C ASM
)

add_subdirectory(kernel)

# Write kernel version
get_target_property(MENIX_BIN_DIR menix RUNTIME_OUTPUT_DIRECTORY)
file(WRITE ${MENIX_BIN_DIR}/version "${CMAKE_PROJECT_VERSION} (${MENIX_ARCH})")
