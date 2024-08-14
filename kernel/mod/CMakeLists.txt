# Modules

set(MENIX_PARENT_CAT menix_modules)
add_library(menix_modules INTERFACE)
include(${MENIX_UTIL_PATH})

# Include all subdirectories with CMakeLists.
file(GLOB_RECURSE subdir_list LIST_DIRECTORIES true "*")

foreach(item ${subdir_list})
	if(IS_DIRECTORY ${item} AND EXISTS "${item}/CMakeLists.txt")
		add_subdirectory(${item})
	endif()
endforeach()

add_library(menix_mod module.c)
target_link_libraries(menix_mod menix_modules)
