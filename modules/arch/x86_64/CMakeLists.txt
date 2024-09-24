# x86 platform drivers

# Include all subdirectories with CMakeLists.
file(GLOB_RECURSE subdir_list LIST_DIRECTORIES true "*")

foreach(item ${subdir_list})
	if(IS_DIRECTORY ${item} AND EXISTS "${item}/CMakeLists.txt")
		add_subdirectory(${item})
	endif()
endforeach()
