cmake_minimum_required(VERSION 3.25)

# Settings
# -------------------
# Target architecture
# Values: x86
set(MENIX_ARCH "x86")
# -------------------

# Circumvent the CMake compiler check since we're using a cross compiler.
set(CMAKE_C_COMPILER_WORKS TRUE CACHE INTERNAL "")
set(CMAKE_CXX_COMPILER_WORKS TRUE CACHE INTERNAL "")

# We use C and Assembly.
project(menix C ASM)

# Add the entry point
add_executable(menix menix/kernel.c)

target_compile_options(menix PUBLIC -ffreestanding)
target_link_options(menix PUBLIC -ffreestanding -nostdlib)

# Include directories on a global level so the implementing sub-libraries have access to them too.
include_directories(menix PUBLIC
	${CMAKE_CURRENT_SOURCE_DIR}/include/
)

# Add the implementation for a given arch.
add_subdirectory(arch/${MENIX_ARCH}/)
target_link_libraries(menix
	menix_arch_${MENIX_ARCH}
	gcc
)