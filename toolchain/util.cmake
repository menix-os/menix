# Common CMake functions

# Setup a new CPU architecture.
# * name		Name of the architecture (and current subdir)
function(add_architecture name)
	set(MENIX_CURRENT_MOD arch_${name} CACHE INTERNAL "")
	add_library(menix_arch_${name} STATIC ${ARGN})

	# Set linker script and common search paths.
	set(CMAKE_EXE_LINKER_FLAGS "-T ${CMAKE_CURRENT_SOURCE_DIR}/${name}.ld -L ${MENIX_SRC} -L ${MENIX_SRC}/toolchain/linker" CACHE INTERNAL "")
	require_option(arch_${name})
endfunction(add_architecture)

# Appends this directory to the Linker Script include search path.
function(add_linker_dir)
	set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -L ${CMAKE_CURRENT_SOURCE_DIR}" CACHE INTERNAL "")
endfunction(add_linker_dir)

# Setup a new module for the build process.
# * name		Name of the module (e.g. example)
# * author		Author of the module (e.g. "John Smith")
# * desc		Short description of the module
# * license		License of the module (e.g. "MIT") or "MAIN", if the project's main license.
# * default		Default configuration value (ON/OFF)
function(add_module name author desc license default)
	set(MENIX_CURRENT_MOD ${name} CACHE INTERNAL "")

	# Generate a config entry if there is none already. If there is, include its values.
	add_option(${MENIX_CURRENT_MOD} BOOL ${default})

	# If the option is not in cache yet, use the default value.
	if(NOT DEFINED CACHE{${MENIX_CURRENT_MOD}})
		set(${MENIX_CURRENT_MOD} ${default} CACHE INTERNAL "")
	endif()

	if(${${MENIX_CURRENT_MOD}} STREQUAL ON)
		# Build as an object library to retain e.g. the module struct since
		# it's technically not referenced anywhere. The linker will discard
		# them otherwise.
		add_library(${MENIX_CURRENT_MOD} OBJECT ${ARGN})

		target_link_libraries(${MENIX_PARENT_CAT} INTERFACE $<TARGET_OBJECTS:${MENIX_CURRENT_MOD}>)

		# If built-in, define MODULE_TYPE to let the module know.
		target_compile_definitions(${MENIX_CURRENT_MOD} PRIVATE MODULE_TYPE='B')

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

		message(STATUS ${optname})
	endif()
endfunction(define_option)

# Configuration function for adding options to a module.
function(add_option optname type default)
	if(type STREQUAL NUMBER)
		set(new_type STRING)
	else()
		set(new_type ${type})
	endif()

	if(${MENIX_HAS_CONFIG} STREQUAL FALSE)
		if(NOT DEFINED ${optname})
			set(${optname} ${default} CACHE ${new_type} "")
			file(APPEND ${MENIX_CONFIG_SRC} "config_option(${optname} ${type} ${default})\n")
		endif()
	endif()

	define_option(${optname} ${type})
endfunction(add_option)

# Overwrite default values for including modules.
# Values: ON, OFF, Literal
function(config_option name type status)
	if(type STREQUAL NUMBER)
		set(type STRING)
	endif()

	# Set a global variable to be evaluated before any other.
	set(${name} "${status}" CACHE ${type} "" FORCE)
endfunction(config_option)

# Automatically select a required option.
function(require_option optname)
	# If it's explicitly turned off, we can't compile.
	if(DEFINED ${optname})
		if(NOT ${${optname}} STREQUAL ON)
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
