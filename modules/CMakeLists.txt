# Modules

set(MENIX_PARENT_CAT menix_builtin)
add_library(${MENIX_PARENT_CAT} INTERFACE)
include(${MENIX_UTIL_PATH})

# Include all subdirectories with CMakeLists.
file(GLOB_RECURSE subdir_list LIST_DIRECTORIES true "*")

foreach(item ${subdir_list})
	if(IS_DIRECTORY ${item} AND EXISTS "${item}/CMakeLists.txt")
		add_subdirectory(${item})
	endif()
endforeach()
