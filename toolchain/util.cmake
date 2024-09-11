# Common CMake functions

# Setup a new CPU architecture.
# * name		Name of the architecture (and current subdir)
function(add_architecture name)
	set(MENIX_CURRENT_MOD arch_${name} CACHE INTERNAL "")
	target_sources(menix PUBLIC ${ARGN})

	# Set linker script and common search paths.
	target_link_options(menix PUBLIC -T ${CMAKE_CURRENT_SOURCE_DIR}/${name}.ld)
	target_link_options(menix PUBLIC "SHELL:-L ${MENIX_SRC}" "SHELL:-L ${MENIX_SRC}/toolchain/linker")

	require_option(arch_${name})
	conflicts_option(arch_*)
endfunction(add_architecture)

# Appends this directory to the Linker Script include search path.
function(add_linker_dir)
	target_link_options(menix PUBLIC "SHELL:-L ${CMAKE_CURRENT_SOURCE_DIR}")
endfunction(add_linker_dir)

# Setup a new module for the build process.
# * name		Name of the module (e.g. example)
# * author		Author of the module (e.g. "John Smith")
# * desc		Short description of the module
# * license		License of the module (e.g. "MIT") or "MAIN", if the project's main license.
# * modular		If the module supports dynamic loading (ON/OFF)
# * default		Default configuration value (ON/MOD/OFF)
function(add_module name author desc license modular default)
	set(MENIX_CURRENT_MOD ${name} CACHE INTERNAL "")

	# Generate a config entry if there is none already. If there is, include its values.
	add_option(${MENIX_CURRENT_MOD} BOOL ${default})

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
	elseif(${${MENIX_CURRENT_MOD}} STREQUAL MOD)
		# Build as a relocatable executable.
		add_executable(${MENIX_CURRENT_MOD} ${ARGN})
		target_compile_options(${MENIX_CURRENT_MOD} PUBLIC -fPIC)
		target_link_options(${MENIX_CURRENT_MOD} PUBLIC -shared)
		set_target_properties(${MENIX_CURRENT_MOD} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/mod/")
		set_target_properties(${MENIX_CURRENT_MOD} PROPERTIES RUNTIME_OUTPUT_NAME "${MENIX_CURRENT_MOD}.ko")

		# If modular, define MODULE_TYPE to let the module know.
		target_compile_definitions(${MENIX_CURRENT_MOD} PRIVATE MODULE_TYPE='M')
	endif()

	# Shared flags
	if(${${MENIX_CURRENT_MOD}} STREQUAL ON OR ${${MENIX_CURRENT_MOD}} STREQUAL MOD)
		target_compile_definitions(${MENIX_CURRENT_MOD} PRIVATE
			MODULE_NAME="${name}"
			MODULE_AUTHOR="${author}"
			MODULE_DESCRIPTION="${desc}"
		)

		# Evaluate module license
		if(${license} STREQUAL "MAIN")
			target_compile_definitions(${MENIX_CURRENT_MOD} PRIVATE MODULE_LICENSE="${MENIX_LICENSE}")
			require_option(license_${MENIX_LICENSE} BOOL)
		else()
			target_compile_definitions(${MENIX_CURRENT_MOD} PRIVATE MODULE_LICENSE="${license}")
			require_option(license_${license} BOOL)
		endif()

		# Add local include directory to search path.
		target_include_directories(${MENIX_CURRENT_MOD} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
	endif()
endfunction(add_module)

# Write an option to the config header.
function(define_option optname type)
	if(DEFINED ${optname})
		# Don't write disabled booleans.
		if(type STREQUAL BOOL)
			if(${${optname}} STREQUAL ON)
				file(APPEND ${MENIX_CONFIG} "#define CONFIG_${optname} 1\n")
			endif()
		elseif(type STREQUAL STRING)
			file(APPEND ${MENIX_CONFIG} "#define CONFIG_${optname} \"${${optname}}\"\n")
		else()
			file(APPEND ${MENIX_CONFIG} "#define CONFIG_${optname} ${${optname}}\n")
		endif()
	endif()
endfunction(define_option)

# Configuration function for adding options to a module.
function(add_option optname type default)
	if(type STREQUAL NUMBER)
		set(new_type STRING)
	else()
		set(new_type ${type})
	endif()

	if(NOT DEFINED ${optname})
		set(${optname} ${default} CACHE ${new_type} "")
		define_option(${optname} ${type})

		if(${MENIX_HAS_CONFIG} STREQUAL FALSE)
			file(APPEND ${MENIX_CONFIG_SRC} "config_option(${optname} ${type} ${default})\n")
		endif()
	endif()
endfunction(add_option)

# Overwrite default values for including modules.
# Values: ON, OFF, Literal
function(config_option name type status)
	if(type STREQUAL NUMBER)
		set(new_type STRING)
	else()
		set(new_type ${type})
	endif()

	# Set a global variable to be evaluated before any other.
	set(${name} "${status}" CACHE ${new_type} "" FORCE)
	define_option(${name} ${type})
endfunction(config_option)

# Automatically select a required option.
function(require_option optname)
	# If it's explicitly turned off, we can't compile.
	if(DEFINED ${optname})
		if(${${optname}} STREQUAL OFF)
			if(${optname} STREQUAL ${MENIX_CURRENT_MOD})
				message(FATAL_ERROR "[!] \"${MENIX_CURRENT_MOD}\" cannot be disabled!\n"
					"-> Enable \"${MENIX_CURRENT_MOD}\"\n")
			else()
				message(FATAL_ERROR "[!] \"${MENIX_CURRENT_MOD}\" requires \"${optname}\" to build, but this was explicitly turned off in the config!\n"
					"-> Either enable \"${optname}\", or disable \"${MENIX_CURRENT_MOD}\".\n")
			endif()
		endif()
	endif()

	add_option(${optname} BOOL ON)
endfunction(require_option)

# This module can not be enabled while another option is active.
function(conflicts_option optname)
	get_cmake_property(all_variables VARIABLES)

	foreach(var ${all_variables})
		string(REGEX MATCH "${optname}" match ${var})

		if(match)
			if(DEFINED ${var})
				# Check if option is enabled and not named the same as our current option.
				if(NOT var STREQUAL MENIX_CURRENT_MOD AND ${${var}} STREQUAL ON AND ${${MENIX_CURRENT_MOD}} STREQUAL ON)
					message(FATAL_ERROR "[!] Module \"${MENIX_CURRENT_MOD}\" conflicts with \"${var}\", you can't have both enabled at once!\n"
						"-> Either disable \"${MENIX_CURRENT_MOD}\" or \"${var}\".\n")
				endif()
			endif()
		endif()
	endforeach()
endfunction(conflicts_option)
