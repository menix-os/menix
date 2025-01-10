# Common CMake functions

# Setup a new CPU architecture.
# * name		Name of the architecture (and current subdir)
function(add_architecture name)
	set(MENIX_CURRENT_MOD arch_${name} CACHE INTERNAL "")
	target_sources(menix PUBLIC ${ARGN})

	# Set linker script and common search paths.
	target_link_options(menix PUBLIC -T ${MENIX_SRC}/toolchain/linker/kernel.ld)
	target_link_options(menix PUBLIC "SHELL:-L ${MENIX_SRC}" "SHELL:-L ${MENIX_SRC}/toolchain/linker")
endfunction(add_architecture)

# Appends this directory to the Linker Script include search path.
function(add_linker_dir)
	target_link_options(menix PUBLIC "SHELL:-L ${CMAKE_CURRENT_SOURCE_DIR}")
endfunction(add_linker_dir)

# Setup a new module for the build process.
# * name		Name of the module (e.g. example)
# * author		Author of the module (e.g. "John Smith")
# * desc		Short description of the module
# * modular		If the module supports dynamic loading (ON/OFF)
# * default		Default configuration value (ON/MOD/OFF)
function(add_module name author desc modular default)
	set(MENIX_CURRENT_MOD ${name} CACHE INTERNAL "")

	# If the option is not in cache yet, use the default value.
	if(NOT DEFINED CACHE{${MENIX_CURRENT_MOD}})
		set(${MENIX_CURRENT_MOD} ${default} CACHE INTERNAL "")
	endif()

	# Check if this module is modular and if not, if we're trying to build it as such.
	if(modular STREQUAL OFF AND ${MENIX_CURRENT_MOD} STREQUAL MOD)
		message(FATAL_ERROR "[!] Module \"${MENIX_CURRENT_MOD}\" can't be built as modular!\n")
	endif()

	if(${${MENIX_CURRENT_MOD}} STREQUAL ON)
		# Build as an object library to retain e.g. the module struct since
		# it's technically not referenced anywhere. The linker will discard
		# them otherwise.
		add_library(${MENIX_CURRENT_MOD} OBJECT ${ARGN})
		target_link_libraries(menix PUBLIC $<TARGET_OBJECTS:${MENIX_CURRENT_MOD}>)

		# If built-in, define MODULE_TYPE to let the module know.
		target_compile_definitions(${MENIX_CURRENT_MOD} PRIVATE MODULE_TYPE='B')
		target_link_libraries(${MENIX_CURRENT_MOD} PRIVATE common_kernel)
	elseif(${${MENIX_CURRENT_MOD}} STREQUAL MOD)
		# Build as an "executable" but in reality, link with -shared.
		add_executable(${MENIX_CURRENT_MOD} ${ARGN})
		set_target_properties(${MENIX_CURRENT_MOD} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/mod/")
		set_target_properties(${MENIX_CURRENT_MOD} PROPERTIES RUNTIME_OUTPUT_NAME "${MENIX_CURRENT_MOD}")

		# If modular, define MODULE_TYPE to let the module know.
		target_compile_definitions(${MENIX_CURRENT_MOD} PRIVATE MODULE_TYPE='M')
		target_link_libraries(${MENIX_CURRENT_MOD} PRIVATE common_ko)
	endif()

	# Shared flags
	if(${${MENIX_CURRENT_MOD}} STREQUAL ON OR ${${MENIX_CURRENT_MOD}} STREQUAL MOD)
		target_link_libraries(${MENIX_CURRENT_MOD} PRIVATE common)
		target_compile_definitions(${MENIX_CURRENT_MOD} PRIVATE
			MODULE_NAME="${name}"
			MODULE_AUTHOR="${author}"
			MODULE_DESCRIPTION="${desc}"
		)

		# Add local include directory to search path.
		target_include_directories(${MENIX_CURRENT_MOD} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
	endif()
endfunction(add_module)
