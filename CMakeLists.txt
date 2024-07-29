# Main configuration file

cmake_minimum_required(VERSION 3.25)

# Project root path.
set(MENIX_SRC ${CMAKE_CURRENT_SOURCE_DIR})

# Project version.
set(MENIX_VERSION 0.1.4)

# Project license.
set(MENIX_LICENSE "LGPL")

# Target architecture, default to x86 if none given.
if(NOT DEFINED MENIX_ARCH)
	set(MENIX_ARCH "x86")
endif()

set(CMAKE_C_STANDARD 23)

# Common build system functionality.
set(MENIX_UTIL_PATH ${MENIX_SRC}/toolchain/util.cmake)
include(${MENIX_UTIL_PATH})

# Configuration file. Create one if it doesn't exist.
set(MENIX_CONFIG_SRC ${MENIX_SRC}/config.cmake)

if(NOT EXISTS ${MENIX_CONFIG_SRC})
	set(MENIX_HAS_CONFIG FALSE)
	file(WRITE ${MENIX_CONFIG_SRC} "# Build configuration\n\n")
else()
	set(MENIX_HAS_CONFIG TRUE)
endif()

# Set toolchain.
include(${MENIX_CONFIG_SRC})
include(${MENIX_SRC}/toolchain/kernel.cmake)
include(${MENIX_SRC}/toolchain/arch/${MENIX_ARCH}.cmake)
include(${MENIX_SRC}/toolchain/arch/defaults.cmake)

project(menix-kernel
	VERSION ${MENIX_VERSION}
	LANGUAGES C ASM
)

add_subdirectory(kernel)
