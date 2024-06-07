# ----------------------
# Common CMake functions
# ----------------------

# Configuration function for including a module in the build process.
function(build_module name)
	set(MENIX_CURRENT_DRV ${name} CACHE INTERNAL "")

	if(NOT "${${MENIX_CURRENT_DRV}}" STREQUAL OFF)
		message(STATUS "${${MENIX_CURRENT_DRV}}\t| " ${name})
	endif()

	add_subdirectory(${name})
endfunction(build_module)

# Setup a new driver for the build process.
# * name		Name of the driver (e.g. example)
# * author		Author of the driver (e.g. "John Smith")
# * license		License of the driver (e.g. "MIT")
# * modular		Can the driver be loaded dynamically or not? (TRUE/FALSE)
# * has_exports	Does the driver export symbols (./mod.h) or not? (TRUE/FALSE)
# * default		Default configuration value (ON/MOD/OFF)
function(add_driver name author license modular has_exports default)
	if(NOT DEFINED MENIX_HAS_CONFIG)
		file(APPEND ${MENIX_CONFIG_SRC} "config_option(${MENIX_CURRENT_DRV} ${default})\n")
	else()
		include(${MENIX_CONFIG_SRC})
	endif()

	# If the option is not in cache yet, use the default value.
	if(NOT DEFINED CACHE{${MENIX_CURRENT_DRV}})
		set(${MENIX_CURRENT_DRV} ${default} CACHE INTERNAL "")
	endif()

	# Check if module is modular and also not built as modular by default.
	if(${modular} STREQUAL FALSE AND ${default} STREQUAL MOD)
		message(FATAL_ERROR "A non-modular module can't have MOD as default!")
	endif()

	if(${modular} STREQUAL FALSE AND ${${MENIX_CURRENT_DRV}} STREQUAL MOD)
		message(FATAL_ERROR "A non-modular module can't be built as a module!")
	endif()

	if(${${MENIX_CURRENT_DRV}} STREQUAL ON OR ${${MENIX_CURRENT_DRV}} STREQUAL MOD)
		# Determine if the module should be linked statically.
		if(${${MENIX_CURRENT_DRV}} STREQUAL ON)
			add_library(${MENIX_CURRENT_DRV} ${ARGN})
			target_link_libraries(${MENIX_PARENT_CAT} INTERFACE ${MENIX_CURRENT_DRV})

			# Global define to check for presence of this module.
			file(APPEND ${MENIX_CONFIG} "#define CFG_${MENIX_CURRENT_DRV}\n")

			# Add exported module functions to the include list.
			if(${has_exports} STREQUAL TRUE)
				file(APPEND ${MENIX_MODULES} "#include \"${CMAKE_CURRENT_SOURCE_DIR}/mod.h\"\n")
			endif()
		else()
			add_executable(${MENIX_CURRENT_DRV} ${ARGN})

			# If compiling as a module, define MENIX_MODULE to let the module know.
			target_compile_definitions(${MENIX_CURRENT_DRV} PRIVATE MODULE)

			# Module should be completely relocatable.
			target_link_options(${MENIX_CURRENT_DRV} PRIVATE -r)
		endif()

		target_compile_definitions(${MENIX_CURRENT_DRV} PRIVATE
			MENIX_DRV_NAME="${name}"
			MENIX_DRV_AUTHOR="${author}"
			MENIX_DRV_LICENSE="${license}"
		)

		# Add local include directory to search path.
		target_include_directories(${MENIX_CURRENT_DRV} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
	endif()
endfunction(add_driver)

function(define_option optname)
	if(${${optname}} STREQUAL ON)
		file(APPEND ${MENIX_CONFIG} "#define CFG_${optname}\n")
	endif()
endfunction(define_option)

# Configuration function for adding options to a module.
function(add_option optname default)
	if(NOT DEFINED CACHE{${optname}})
		set(${optname} ${default} CACHE INTERNAL "")
	endif()

	define_option(${optname})
endfunction(add_option)

# Overwrite default values for including modules.
# Values: ON, OFF, MOD
function(config_option name status)
	# Set a global variable to be evaluated before any other.
	set(${name} ${status} CACHE INTERNAL "")
endfunction(config_option)

# Automatically select required option.
function(require_option optname)
	# If the option is not explicitly turned off, just define it.
	if(NOT DEFINED CACHE{${optname}})
		set(${optname} ON CACHE INTERNAL "")

	# If it's explicitly turned off, we can't compile.
	elseif(NOT ${optname} STREQUAL ON)
		message(FATAL_ERROR "Something requires \"${optname}\" to build, but this was explicitly turned off in the config!\n"
			"You might want to rebuild the cache.")
	endif()

	message(STATUS "AUTO | ${optname}")
	define_option(${optname})
endfunction(require_option)
