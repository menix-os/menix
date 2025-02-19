# Kernel modules and drivers
# Add new categories at the end of this file

add_library(menix_builtin INTERFACE)

# Setup a new module for the build process.
# * name		Name of the module (e.g. example)
# * author		Author of the module (e.g. "John Smith")
# * desc		Short description of the module
# * modular		If the module supports dynamic loading (ON/OFF)
# * default		Default configuration value (ON/MOD/OFF)
function(add_module name author desc modular default)
	# If the option is not overridden, use the default value.
	if(NOT DEFINED ${name})
		set(${name} ${default})
	endif()

	# Check if this module is modular and if not, if we're trying to build it as such.
	if(modular STREQUAL OFF AND name STREQUAL MOD)
		message(FATAL_ERROR "[!] Module \"${name}\" can't be built as modular!\n")
	endif()

	if(${name} STREQUAL ON)
		# Build as an object library to retain e.g. the module struct since
		# it's technically not referenced anywhere. The linker will discard
		# them otherwise.
		add_library(${name} OBJECT ${ARGN})
		target_link_libraries(menix PUBLIC $<TARGET_OBJECTS:${name}>)

		# If built-in, define MODULE_TYPE to let the module know.
		target_compile_definitions(${name} PRIVATE MODULE_TYPE='B')
		target_link_libraries(${name} PRIVATE common_kernel)
	elseif(${name} STREQUAL MOD)
		# Build as an "executable" but in reality, link with -shared.
		add_executable(${name} ${ARGN})
		set_target_properties(${name} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/modules")
		set_target_properties(${name} PROPERTIES RUNTIME_OUTPUT_NAME "${name}")

		# If modular, define MODULE_TYPE to let the module know.
		target_compile_definitions(${name} PRIVATE MODULE_TYPE='M')
		target_link_libraries(${name} PRIVATE common_ko)
	endif()

	# Shared flags
	if(${name} STREQUAL ON OR ${name} STREQUAL MOD)
		target_link_libraries(${name} PRIVATE common)
		target_compile_definitions(${name} PRIVATE
			MODULE_NAME="${name}"
			MODULE_AUTHOR="${author}"
			MODULE_DESCRIPTION="${desc}"
		)

		# Add local include directory to search path.
		target_include_directories(${name} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
	endif()
endfunction(add_module)

# Architecture dependent drivers
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/arch/${MENIX_ARCH})
	add_subdirectory(arch/${MENIX_ARCH})
endif()

# Shared headers
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

add_subdirectory(block)
add_subdirectory(gpu)
