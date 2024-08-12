# File systems and VFS

set(MENIX_PARENT_CAT menix_file_systems)
add_library(menix_file_systems INTERFACE)
include(${MENIX_UTIL_PATH})

# -----

# -----
add_library(menix_fs
	vfs.c
)
target_link_libraries(menix_fs menix_file_systems)
