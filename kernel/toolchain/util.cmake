# ----------------------
# Common CMake functions
# ----------------------

# Configuration function for including a module in the build process.
function(build_module name)
	set(MENIX_CUR_MODULE ${name} CACHE INTERNAL "")
	message(STATUS "${${MENIX_CUR_MODULE}}\t| " ${name})
	add_subdirectory(${name})
endfunction(build_module)

# Setup a new module.
function(add_module name author license modular default has_exports)
	# If the option is not in cache yet, use the default value.
	if (NOT DEFINED CACHE{${MENIX_CUR_MODULE}})
		set(${MENIX_CUR_MODULE} ${default} CACHE INTERNAL "")
	endif()

	# Check if module is modular and also not built as modular by default.
	if (${modular} STREQUAL FALSE AND ${default} STREQUAL MOD)
		message(FATAL_ERROR "A non-modular module can't have MOD as default!")
	endif()
	if (${modular} STREQUAL FALSE AND ${${MENIX_CUR_MODULE}} STREQUAL MOD)
		message(FATAL_ERROR "A non-modular module can't be built as a module!")
	endif()

	if(${${MENIX_CUR_MODULE}} STREQUAL ON OR ${${MENIX_CUR_MODULE}} STREQUAL MOD)
		# Determine if the module should be linked statically.
		if(${${MENIX_CUR_MODULE}} STREQUAL ON)
			add_library(${MENIX_CUR_MODULE} ${ARGN})
			target_link_libraries(${MENIX_PARENT_CAT} INTERFACE ${MENIX_CUR_MODULE})
			# Global define to check for presence of this module.
			file(APPEND ${MENIX_CONFIG} "#define CFG_${MENIX_CUR_MODULE}\n")
			# Add exported module functions to the include list.
			if (${has_exports} STREQUAL TRUE)
				file(APPEND ${MENIX_MODULES} "#include \"${CMAKE_CURRENT_SOURCE_DIR}/mod.h\"\n")
			endif()
		else()
			add_executable(${MENIX_CUR_MODULE} ${ARGN})
			# If compiling as a module, define MENIX_MODULE to let the module know.
			target_compile_definitions(${MENIX_CUR_MODULE} PRIVATE MODULE)
			# Module should be completely relocatable.
			target_link_options(${MENIX_CUR_MODULE} PRIVATE -r)
		endif()

		target_compile_definitions(${MENIX_CUR_MODULE} PRIVATE
			MODULE_NAME="${name}"
			MODULE_AUTHOR="${author}"
			MODULE_LICENSE="${license}"
		)

		# Add local include directory to search path.
		target_include_directories(${MENIX_CUR_MODULE} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
	endif()
endfunction(add_module)

function(define_option optname)
if(${${optname}} STREQUAL ON)
	file(APPEND ${MENIX_CONFIG} "#define CFG_${optname}\n")
endif()
endfunction(define_option)

# Configuration function for adding options to a module.
function(add_option optname default)
	if (NOT DEFINED CACHE{${optname}})
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
	if (NOT DEFINED CACHE{${optname}})
		message(STATUS "     | - Auto-selecting: ${optname}")
		set(${optname} ON CACHE INTERNAL "")
		define_option(${optname})
	# If it's explicitly turned off, we can't compile.
	elseif (NOT ${optname} STREQUAL ON)
		message(FATAL_ERROR "\"${MENIX_CUR_MODULE}\" requires \"${optname}\" to build, but this was explicitly turned off!")
	endif()
endfunction(require_option)
