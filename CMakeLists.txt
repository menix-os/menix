# Main configuration file

cmake_minimum_required(VERSION 3.29)

# Project root path.
set(MENIX_SRC ${CMAKE_CURRENT_SOURCE_DIR})

# Project version.
set(MENIX_VERSION 0.1.5)

# Project license.
set(MENIX_LICENSE "LGPL")

# Target architecture, default to x86 if none given.
if(NOT DEFINED MENIX_ARCH)
	set(MENIX_ARCH "x86")
endif()

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

set(CMAKE_C_STANDARD 23)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(menix
	VERSION ${MENIX_VERSION}
	LANGUAGES C ASM
)

add_subdirectory(kernel)
