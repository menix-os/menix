# Main configuration file

cmake_minimum_required(VERSION 3.25)

# Project root path.
set(MENIX_SRC ${CMAKE_CURRENT_SOURCE_DIR})

# Project version.
set(MENIX_RELEASE "0.0.1")
string(TIMESTAMP MENIX_VERSION "%a %b %d %H:%M:%S UTC %Y" UTC)

if(NOT(CMAKE_BUILD_TYPE STREQUAL "Release" OR CMAKE_BUILD_TYPE STREQUAL "MinSizeRel"))
	set(MENIX_VERSION "${MENIX_VERSION} (Unoptimized)")
endif()

# Target architecture, default to x86 if none given.
if(NOT DEFINED MENIX_ARCH)
	set(MENIX_ARCH "x86_64")
endif()

set(CMAKE_C_STANDARD 23)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_SYSTEM_NAME Generic)

project(menix LANGUAGES C ASM)

add_executable(menix)

# Targets for common compile and link flags
add_library(common INTERFACE)
add_library(common_kernel INTERFACE)
add_library(common_server INTERFACE)

target_link_libraries(menix PRIVATE common common_kernel)

include(${MENIX_SRC}/toolchain/arch/${MENIX_ARCH}.cmake) # Include architecture specific flags
include(${MENIX_SRC}/toolchain/compiler/${CMAKE_C_COMPILER_ID}.cmake) # Include compiler specific flags
include(${MENIX_SRC}/toolchain/compiler/common.cmake) # Include common flags

target_compile_definitions(common_kernel INTERFACE MENIX_ARCH="${MENIX_ARCH}")
target_compile_definitions(common_kernel INTERFACE MENIX_RELEASE="${MENIX_RELEASE}")
target_compile_definitions(common_kernel INTERFACE MENIX_VERSION="${MENIX_VERSION}")

# Global include directories.
target_include_directories(common_kernel INTERFACE ${MENIX_SRC}/include/)
target_include_directories(common_kernel INTERFACE ${MENIX_SRC}/include/klibc/)
target_include_directories(common_kernel INTERFACE ${MENIX_SRC}/kernel/arch/${MENIX_ARCH}/include/)

add_subdirectory(kernel)
add_subdirectory(servers)
