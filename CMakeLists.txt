# Main configuration file

cmake_minimum_required(VERSION 3.25)

set(CMAKE_C_STANDARD 23)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_SYSTEM_NAME Generic)

project(menix LANGUAGES C ASM)

# Target architecture, default to x86_64 if none given.
if(NOT DEFINED MENIX_ARCH)
	set(MENIX_ARCH "x86_64")
endif()

# Project version and release.
set(MENIX_RELEASE "0.0.1")
string(TIMESTAMP MENIX_VERSION "%a %b %d %H:%M:%S UTC %Y" UTC)

# Target for common compile and link flags
add_library(common INTERFACE)
add_library(common_kernel INTERFACE)
add_library(common_ko INTERFACE)

# Set toolchain.
include(toolchain/arch/${MENIX_ARCH}.cmake)
include(toolchain/compiler/${CMAKE_C_COMPILER_ID}.cmake)
include(toolchain/compiler/common.cmake)

target_compile_definitions(common INTERFACE MENIX_ARCH="${MENIX_ARCH}")
target_compile_definitions(common INTERFACE MENIX_RELEASE="${MENIX_RELEASE}")
target_compile_definitions(common INTERFACE MENIX_VERSION="${MENIX_VERSION}")

# Global include directories.
include_directories(include/)
include_directories(include/klibc/)
include_directories(kernel/arch/${MENIX_ARCH}/include/)

add_executable(menix)
target_link_libraries(menix PUBLIC common common_kernel)
target_link_libraries(menix PUBLIC menix_builtin)

add_subdirectory(kernel)
add_subdirectory(modules)
