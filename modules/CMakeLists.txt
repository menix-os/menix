# Modules

set(MENIX_PARENT_CAT menix_builtin)
add_library(${MENIX_PARENT_CAT} INTERFACE)
include(${MENIX_UTIL_PATH})

# Include all subdirectories with CMakeLists.
file(GLOB_RECURSE subdir_list LIST_DIRECTORIES true "*")

# Prepare module build folder so we can create an archive, even if there's no modules to be packed.
file(MAKE_DIRECTORY "${CMAKE_BINARY_DIR}/bin/mod")

include_directories(${CMAKE_CURRENT_SOURCE_DIR})

foreach(item ${subdir_list})
	if(IS_DIRECTORY ${item} AND EXISTS "${item}/CMakeLists.txt")
		add_subdirectory(${item})
	endif()
endforeach()
