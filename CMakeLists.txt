# Main configuration file

cmake_minimum_required(VERSION 3.25)

# Project root path.
set(MENIX_SRC ${CMAKE_CURRENT_SOURCE_DIR})

# Project version.
set(MENIX_RELEASE "0.0.1")
string(TIMESTAMP MENIX_VERSION "%a %b %d %H:%M:%S UTC %Y" UTC)

# Get the current git commit hash.
execute_process(
	COMMAND git log -1 --format=%h
	WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
	OUTPUT_VARIABLE GIT_HASH
	OUTPUT_STRIP_TRAILING_WHITESPACE
)
set(MENIX_VERSION "${MENIX_VERSION} (${GIT_HASH})")

# Project license.
set(MENIX_LICENSE "LGPL")

# Target architecture, default to x86 if none given.
if(NOT DEFINED MENIX_ARCH)
	set(MENIX_ARCH "x86_64")
endif()

# Common build system functionality.
set(MENIX_UTIL_PATH ${MENIX_SRC}/toolchain/util.cmake)
include(${MENIX_UTIL_PATH})

set(CMAKE_C_STANDARD 23)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_SYSTEM_NAME Generic)

project(menix LANGUAGES C CXX ASM)

add_executable(menix)

# Target for common compile and link flags
add_library(common INTERFACE)
add_library(common_kernel INTERFACE)
add_library(common_ko INTERFACE)

target_link_libraries(menix PUBLIC common common_kernel)

# Set toolchain.
include(${MENIX_SRC}/toolchain/arch/${MENIX_ARCH}.cmake)
include(${MENIX_SRC}/toolchain/compiler/${CMAKE_C_COMPILER_ID}.cmake)
include(${MENIX_SRC}/toolchain/compiler/common.cmake)

target_compile_definitions(common INTERFACE MENIX_BITS=${MENIX_BITS})
target_compile_definitions(common INTERFACE MENIX_ARCH="${MENIX_ARCH}")
target_compile_definitions(common INTERFACE MENIX_RELEASE="${MENIX_RELEASE}")
target_compile_definitions(common INTERFACE MENIX_VERSION="${MENIX_VERSION}")

# Global include directories.
include_directories(${MENIX_SRC}/include/)
include_directories(${MENIX_SRC}/include/klibc/)
include_directories(${MENIX_SRC}/kernel/arch/${MENIX_ARCH}/include/)

add_subdirectory(kernel)
add_subdirectory(modules)

# Link built-in modules
target_link_libraries(menix PUBLIC menix_builtin)
